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
        // call void @__quantum__qis__... または %res = call %Result* @...
        // 戻り値がある場合とない場合の両方に対応
        let re_call = Regex::new(r"(?:%\w+\s*=\s*)?call\s+(?:void|%Result\*)\s+@(?P<func>__quantum__qis__[^\s(]+)\((?P<args>[^)]*)\)").unwrap();

        for line in &self.lines {
            if let Some(caps) = re_call.captures(line) {
                let func_name = &caps["func"];
                let args_str = &caps["args"];
                
                let args = self.parse_args(args_str);
                bridge.call_qis(func_name, args)?;
            }
        }
        Ok(())
    }

    fn parse_args(&self, args_str: &str) -> Vec<usize> {
        args_str.split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| self.extract_id(s))
            .collect()
    }

    fn extract_id(&self, s: &str) -> usize {
        if s.contains("null") {
            0
        } else if s.contains("inttoptr") {
            // 例: %Qubit* inttoptr (i64 1 to %Qubit*)
            let re_digits = Regex::new(r"i64\s+(\d+)").unwrap();
            if let Some(caps) = re_digits.captures(s) {
                caps[1].parse().unwrap_or(0)
            } else {
                0
            }
        } else {
            // 数値変換を試みる
            s.chars().filter(|c| c.is_digit(10)).collect::<String>()
                .parse().unwrap_or(0)
        }
    }
}
