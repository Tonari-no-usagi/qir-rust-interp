use crate::error::Result;
use std::path::Path;
use std::fs::read_to_string;
use regex::Regex;

pub struct QirParser {
    lines: Vec<String>,
}

impl QirParser {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = read_to_string(path)?;
        let lines = content.lines().map(|s| s.to_string()).collect();
        Ok(Self { lines })
    }

    pub fn run_interpreter(&self, bridge: &mut crate::qis_bridge::QisBridge) -> Result<()> {
        let mut labels = std::collections::HashMap::new();
        // 1. ラベルの事前スキャン
        for (i, line) in self.lines.iter().enumerate() {
            let line = line.trim();
            if line.ends_with(':') {
                let label_name = &line[..line.len()-1];
                labels.insert(label_name.to_string(), i);
            }
        }

        let re_call = Regex::new(r"(?:(?P<ret>%\w+)\s*=\s*)?call\s+(?:void|i1|%Result\*)\s+@(?P<func>(?:__quantum__qis__|__quantum__rt__)[^\s(]+)\((?P<args>[^)]*)\)").unwrap();
        let re_br_cond = Regex::new(r"br\s+i1\s+(?P<cond>%\w+),\s+label\s+%(?P<then>\w+),\s+label\s+%(?P<else>\w+)").unwrap();
        let re_br_uncond = Regex::new(r"br\s+label\s+%(?P<dest>\w+)").unwrap();
        let re_assign = Regex::new(r"(?P<var>%\w+)\s*=\s*(?P<val>.*)").unwrap();

        let mut pc = 0;
        let mut variables: std::collections::HashMap<String, bool> = std::collections::HashMap::new();

        while pc < self.lines.len() {
            let line = self.lines[pc].trim();
            
            if line.is_empty() || line.ends_with(':') || line.starts_with(';') {
                pc += 1;
                continue;
            }

            // 条件分岐
            if let Some(caps) = re_br_cond.captures(line) {
                let cond_var = &caps["cond"];
                let is_true = *variables.get(cond_var).unwrap_or(&false);
                let dest = if is_true { &caps["then"] } else { &caps["else"] };
                pc = *labels.get(dest).ok_or_else(|| crate::error::QirError::ParseError(format!("Label not found: {}", dest)))?;
                continue;
            }
            
            // 無条件分岐
            if let Some(caps) = re_br_uncond.captures(line) {
                let dest = &caps["dest"];
                pc = *labels.get(dest).ok_or_else(|| crate::error::QirError::ParseError(format!("Label not found: {}", dest)))?;
                continue;
            }

            // 関数呼び出し
            if let Some(caps) = re_call.captures(line) {
                let func_name = &caps["func"];
                let args_str = &caps["args"];
                let args = self.parse_args(args_str);

                let result = bridge.call_qis(func_name, args)?;
                
                // 戻り値がある場合は変数に格納
                if let Some(ret_var) = caps.name("ret") {
                    if let Some(val) = result {
                        variables.insert(ret_var.as_str().to_string(), val);
                    }
                }
            }

            pc += 1;
        }
        Ok(())
    }

    fn parse_args(&self, args_str: &str) -> Vec<usize> {
        args_str.split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| self.extract_value(s))
            .collect()
    }

    fn extract_value(&self, s: &str) -> usize {
        let s = s.trim();
        if s.contains("null") {
            0
        } else if s.contains("double") {
            // double 1.570796e+00 のような形式
            let parts: Vec<&str> = s.split_whitespace().collect();
            if parts.len() >= 2 {
                let val: f64 = parts[1].parse().unwrap_or(0.0);
                val.to_bits() as usize
            } else {
                0
            }
        } else if s.contains("inttoptr") {
            // 例: %Qubit* inttoptr (i64 1 to %Qubit*)
            let re_digits = Regex::new(r"i64\s+(\d+)").unwrap();
            if let Some(caps) = re_digits.captures(s) {
                caps[1].parse().unwrap_or(0)
            } else {
                0
            }
        } else {
            // 数値変換を試みる (i64 0 など)
            // スペースで分割して最後の要素に注目し、数値以外の文字を排除
            s.split_whitespace()
                .last()
                .map(|p| p.chars().filter(|c| c.is_digit(10) || *c == '.').collect::<String>())
                .and_then(|s| s.parse::<f64>().ok())
                .map(|f| f as usize)
                .unwrap_or(0)
        }
    }
}
