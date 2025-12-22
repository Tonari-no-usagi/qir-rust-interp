use std::collections::HashMap;

/// QIRの不透明ポインタ (%Qubit*, %Result*) と内部インデックスのマッピングを管理する
pub struct QirMemory {
    qubit_map: HashMap<usize, usize>,
    result_map: HashMap<usize, usize>,
    next_qubit_id: usize,
    next_result_id: usize,
}

impl QirMemory {
    pub fn new() -> Self {
        Self {
            qubit_map: HashMap::new(),
            result_map: HashMap::new(),
            next_qubit_id: 0,
            next_result_id: 0,
        }
    }

    /// ポインタアドレス（またはQIR上のID）から内部量子ビットインデックスを取得
    pub fn get_qubit(&mut self, addr: usize) -> usize {
        *self.qubit_map.entry(addr).or_insert_with(|| {
            let id = self.next_qubit_id;
            self.next_qubit_id += 1;
            id
        })
    }

    /// ポインタアドレスから内部結果インデックスを取得
    pub fn get_result(&mut self, addr: usize) -> usize {
        *self.result_map.entry(addr).or_insert_with(|| {
            let id = self.next_result_id;
            self.next_result_id += 1;
            id
        })
    }
}
