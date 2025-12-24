use ndarray::Array1;
use num_complex::Complex64;
use crate::error::{QirError, Result};
use rand::Rng;

pub struct Simulator {
    /// 状態ベクトル
    state: Array1<Complex64>,
    /// 量子ビット数
    num_qubits: usize,
}

impl Simulator {
    pub fn new(num_qubits: usize) -> Self {
        let size = 1 << num_qubits;
        let mut state = Array1::zeros(size);
        state[0] = Complex64::new(1.0, 0.0); // |0...0> 状態に初期化
        Self { state, num_qubits }
    }

    /// 指定された量子ビットに1量子ビットゲートを適用する
    pub fn apply_gate_1q(&mut self, target: usize, gate_matrix: [[Complex64; 2]; 2]) -> Result<()> {
        if target >= self.num_qubits {
            return Err(QirError::SimulatorError(format!("Qubit index {} out of range", target)));
        }

        let shift = target;
        let mask = 1 << shift;

        for i in 0..(1 << self.num_qubits) {
            if (i & mask) == 0 {
                let j = i | mask;
                let c0 = self.state[i];
                let c1 = self.state[j];

                self.state[i] = gate_matrix[0][0] * c0 + gate_matrix[0][1] * c1;
                self.state[j] = gate_matrix[1][0] * c0 + gate_matrix[1][1] * c1;
            }
        }

        Ok(())
    }

    /// CNOTゲートを適用する
    pub fn apply_cnot(&mut self, control: usize, target: usize) -> Result<()> {
        if control >= self.num_qubits || target >= self.num_qubits {
            return Err(QirError::SimulatorError("Qubit index out of range".into()));
        }

        let c_mask = 1 << control;
        let t_mask = 1 << target;

        for i in 0..(1 << self.num_qubits) {
            // 制御ビットが1かつ、targetビットが0のインデックスを見つけて、targetビットが1のものとスワップ
            if (i & c_mask) != 0 && (i & t_mask) == 0 {
                let j = i | t_mask;
                let tmp = self.state[i];
                self.state[i] = self.state[j];
                self.state[j] = tmp;
            }
        }

        Ok(())
    }

    /// CZゲートを適用する
    pub fn apply_cz(&mut self, control: usize, target: usize) -> Result<()> {
        if control >= self.num_qubits || target >= self.num_qubits {
            return Err(QirError::SimulatorError("Qubit index out of range".into()));
        }

        let c_mask = 1 << control;
        let t_mask = 1 << target;

        for i in 0..(1 << self.num_qubits) {
            if (i & c_mask) != 0 && (i & t_mask) != 0 {
                self.state[i] = -self.state[i];
            }
        }

        Ok(())
    }

    /// Rxゲートを適用する
    pub fn apply_rx(&mut self, target: usize, theta: f64) -> Result<()> {
        let (sin, cos) = (theta / 2.0).sin_cos();
        let matrix = [
            [Complex64::new(cos, 0.0), Complex64::new(0.0, -sin)],
            [Complex64::new(0.0, -sin), Complex64::new(cos, 0.0)],
        ];
        self.apply_gate_1q(target, matrix)
    }

    /// Ryゲートを適用する
    pub fn apply_ry(&mut self, target: usize, theta: f64) -> Result<()> {
        let (sin, cos) = (theta / 2.0).sin_cos();
        let matrix = [
            [Complex64::new(cos, 0.0), Complex64::new(-sin, 0.0)],
            [Complex64::new(sin, 0.0), Complex64::new(cos, 0.0)],
        ];
        self.apply_gate_1q(target, matrix)
    }

    /// Rzゲートを適用する
    pub fn apply_rz(&mut self, target: usize, theta: f64) -> Result<()> {
        let (sin, cos) = (theta / 2.0).sin_cos();
        let matrix = [
            [Complex64::new(cos, -sin), Complex64::new(0.0, 0.0)],
            [Complex64::new(0.0, 0.0), Complex64::new(cos, sin)],
        ];
        self.apply_gate_1q(target, matrix)
    }

    /// 測定を行う
    pub fn measure(&mut self, qubit: usize) -> Result<bool> {
        if qubit >= self.num_qubits {
            return Err(QirError::SimulatorError(format!("Qubit index {} out of range", qubit)));
        }

        let mask = 1 << qubit;
        let mut p0 = 0.0;

        for i in 0..(1 << self.num_qubits) {
            if (i & mask) == 0 {
                p0 += self.state[i].norm_sqr();
            }
        }

        let mut rng = rand::thread_rng();
        let p_true = (1.0 - p0).clamp(0.0, 1.0);
        let result = rng.gen_bool(p_true);

        // 状態の崩壊と正規化
        let p = if result { 1.0 - p0 } else { p0 };
        if p < 1e-15 {
            // 確率がほぼ0の場合は特殊処理（理論上は発生しないはずだが浮動小数点の精度的に）
            return Ok(result);
        }
        
        let norm = p.sqrt();
        for i in 0..(1 << self.num_qubits) {
            let matches = (i & mask) != 0;
            if matches == result {
                self.state[i] = self.state[i] / norm;
            } else {
                self.state[i] = Complex64::new(0.0, 0.0);
            }
        }

        Ok(result)
    }

    pub fn get_state(&self) -> &Array1<Complex64> {
        &self.state
    }

    /// 現在の状態の確率分布を表示する（デバッグ用）
    pub fn print_probabilities(&self) {
        println!("Current Probability Distribution:");
        for (i, c) in self.state.iter().enumerate() {
            let p = c.norm_sqr();
            if p > 1e-6 {
                println!("|{:0>width$b}>: {:.4}", i, p, width = self.num_qubits);
            }
        }
    }
}
