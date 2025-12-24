use std::collections::HashMap;

/// QIRの不透明ポインタ (%Qubit*, %Result*) と内部インデックスのマッピングを管理する
pub struct QirMemory {
    /// 測定結果の値を保持 (%Result* の位置に bool を格納)
    result_values: HashMap<usize, bool>,
}

impl QirMemory {
    pub fn new() -> Self {
        Self {
            result_values: HashMap::new(),
        }
    }

    /// ポインタアドレスから内部量子ビットインデックスを取得
    pub fn get_qubit(&mut self, addr: usize) -> usize {
        addr
    }

    /// ポインタアドレス（または結果ポインタのアドレス）に測定値を保存
    pub fn set_result_value(&mut self, addr: usize, value: bool) {
        self.result_values.insert(addr, value);
    }

    /// 測定値を取得
    pub fn get_result_value(&self, addr: usize) -> bool {
        *self.result_values.get(&addr).unwrap_or(&false)
    }
}
