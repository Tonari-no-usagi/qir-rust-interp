use crate::simulator::Simulator;
use crate::memory::QirMemory;
use crate::error::Result;
use num_complex::Complex64;
use std::f64::consts::FRAC_1_SQRT_2;

pub struct QisBridge<'a> {
    simulator: &'a mut Simulator,
    memory: &'a mut QirMemory,
}

impl<'a> QisBridge<'a> {
    pub fn new(simulator: &'a mut Simulator, memory: &'a mut QirMemory) -> Self {
        Self { simulator, memory }
    }

    pub fn get_result_value(&self, addr: usize) -> bool {
        self.memory.get_result_value(addr)
    }

    pub fn call_qis(&mut self, func_name: &str, args: Vec<usize>) -> Result<Option<bool>> {
        match func_name {
            "__quantum__qis__h__body" => {
                let qubit = self.memory.get_qubit(args[0]);
                let h_matrix = [
                    [Complex64::new(FRAC_1_SQRT_2, 0.0), Complex64::new(FRAC_1_SQRT_2, 0.0)],
                    [Complex64::new(FRAC_1_SQRT_2, 0.0), Complex64::new(-FRAC_1_SQRT_2, 0.0)],
                ];
                self.simulator.apply_gate_1q(qubit, h_matrix)?;
                Ok(None)
            }
            "__quantum__qis__x__body" => {
                let qubit = self.memory.get_qubit(args[0]);
                let x_matrix = [
                    [Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0)],
                    [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
                ];
                self.simulator.apply_gate_1q(qubit, x_matrix)?;
                Ok(None)
            }
            "__quantum__qis__y__body" => {
                let qubit = self.memory.get_qubit(args[0]);
                let y_matrix = [
                    [Complex64::new(0.0, 0.0), Complex64::new(0.0, -1.0)],
                    [Complex64::new(0.0, 1.0), Complex64::new(0.0, 0.0)],
                ];
                self.simulator.apply_gate_1q(qubit, y_matrix)?;
                Ok(None)
            }
            "__quantum__qis__z__body" => {
                let qubit = self.memory.get_qubit(args[0]);
                let z_matrix = [
                    [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
                    [Complex64::new(0.0, 0.0), Complex64::new(-1.0, 0.0)],
                ];
                self.simulator.apply_gate_1q(qubit, z_matrix)?;
                Ok(None)
            }
            "__quantum__qis__s__body" => {
                let qubit = self.memory.get_qubit(args[0]);
                let s_matrix = [
                    [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
                    [Complex64::new(0.0, 0.0), Complex64::new(0.0, 1.0)],
                ];
                self.simulator.apply_gate_1q(qubit, s_matrix)?;
                Ok(None)
            }
            "__quantum__qis__t__body" => {
                let qubit = self.memory.get_qubit(args[0]);
                let t_matrix = [
                    [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
                    [Complex64::new(0.0, 0.0), Complex64::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2)],
                ];
                self.simulator.apply_gate_1q(qubit, t_matrix)?;
                Ok(None)
            }
            "__quantum__qis__cnot__body" => {
                let control = self.memory.get_qubit(args[0]);
                let target = self.memory.get_qubit(args[1]);
                self.simulator.apply_cnot(control, target)?;
                Ok(None)
            }
            "__quantum__qis__cz__body" => {
                let control = self.memory.get_qubit(args[0]);
                let target = self.memory.get_qubit(args[1]);
                self.simulator.apply_cz(control, target)?;
                Ok(None)
            }
            "__quantum__qis__rx__body" => {
                let theta = f64::from_bits(args[0] as u64);
                let qubit = self.memory.get_qubit(args[1]);
                self.simulator.apply_rx(qubit, theta)?;
                Ok(None)
            }
            "__quantum__qis__ry__body" => {
                let theta = f64::from_bits(args[0] as u64);
                let qubit = self.memory.get_qubit(args[1]);
                self.simulator.apply_ry(qubit, theta)?;
                Ok(None)
            }
            "__quantum__qis__rz__body" => {
                let theta = f64::from_bits(args[0] as u64);
                let qubit = self.memory.get_qubit(args[1]);
                self.simulator.apply_rz(qubit, theta)?;
                Ok(None)
            }
            "__quantum__qis__mz__body" => {
                let qubit = self.memory.get_qubit(args[0]);
                // QIR Base Profile/Adaptive Profile互換のため、戻り値として結果を返す。
                // またResult管理が必要な場合は、memory.rs 経由で保存することも可能。
                let res = self.simulator.measure(qubit)?;
                Ok(Some(res))
            }
            "__quantum__qis__read_result__body" => {
                let result_addr = args[0];
                let val = self.memory.get_result_value(result_addr);
                println!("Reading result from {}: {}", result_addr, val);
                Ok(Some(val))
            }
            _ => {
                println!("Warning: Unsupported QIS function: {}", func_name);
                Ok(None)
            }
        }
    }
}
