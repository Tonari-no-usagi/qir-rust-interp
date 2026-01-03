pub mod error;
pub mod memory;
pub mod simulator;
pub mod parser;
pub mod qis_bridge;

use pyo3::prelude::*;
use std::collections::HashMap;
use crate::parser::QirParser;
use crate::simulator::Simulator;
use crate::memory::QirMemory;
use crate::qis_bridge::QisBridge;

/// QIRファイルを実行し、最終的な確率分布を Python の辞書で返す
#[pyfunction]
#[pyo3(signature = (file_path, qubits=10))]
fn run_qir(file_path: String, qubits: usize) -> PyResult<HashMap<String, f64>> {
    let parser = QirParser::from_file(&file_path)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;

    let mut simulator = Simulator::new(qubits);
    let mut memory = QirMemory::new();
    let mut bridge = QisBridge::new(&mut simulator, &mut memory);

    parser.run_interpreter(&mut bridge)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;

    let mut results = HashMap::new();
    let state = simulator.get_state();
    for (i, c) in state.iter().enumerate() {
        let p = c.norm_sqr();
        if p > 1e-10 {
            let bit_string = format!("{:0>width$b}", i, width = qubits);
            results.insert(bit_string, p);
        }
    }

    Ok(results)
}

/// Python モジュールの定義
#[pymodule]
fn qir_rust_interp(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run_qir, m)?)?;
    Ok(())
}
