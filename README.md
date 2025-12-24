# qir-rust-interp

**A lightweight, zero-dependency QIR emulator and interpreter built in Rust.**

`qir-rust-interp` is a tool designed to execute Quantum Intermediate Representation (QIR). It aims to provide a fast development experience for testing and learning quantum algorithms without requiring heavy dependencies like the LLVM toolchain.

---

## ✨ Features

- **No LLVM Dependency**: Runs even if LLVM is not installed on your system.
- **Pure Rust Simulator**: A state-vector simulator built from scratch using `ndarray`.
- **Quantum Control Support**: Supports measurement-based conditional branching (`br` instructions and labels).
- **Expanded QIS Support**:
  - Standard Gates: `H`, `X`, `Y`, `Z`, `S`, `T`
  - Rotation Gates: `Rx`, `Ry`, `Rz` (Any angle supported)
  - 2Q Gates: `CNOT`, `CZ`
  - Measurement: Includes probabilistic state collapse and result storage for classical control.
- **Instant Execution**: Directly interprets textual LLVM IR (`.ll`) files.

---

## 🛠 Technical Highlights

- **Fast Parsing**: Uses optimized regular expressions to extract gates and control flow directly from `.ll` files.
- **Robust Simulation**: Handles floating-point precision issues in measurement probabilities with automated clamping.
- **Modular Design**: Clearly separated Parser, QIS Bridge, and Simulator layers for easy extensibility.

---

## 🏃 Quick Start

### 1. Installation
As long as you have a Rust environment, you can clone and build it immediately.

```bash
git clone https://github.com/Tonari-no-usagi/qir-rust-interp.git
cd qir-rust-interp
cargo build
```

### 2. Run Samples

#### Bell State
Generates a Bell state (quantum entanglement) using H and CNOT gates.
```bash
cargo run -- samples/bell.ll --qubits 2
```

#### Quantum Teleportation (Classical Control & Measurement)
Simulates a full teleportation circuit including measurement-based feed-forward (X and Z corrections).
```bash
cargo run -- samples/teleportation.ll --qubits 3
```

#### Rotation Gates
Demonstrates arbitrary rotation gates (Rx, Ry, Rz).
```bash
cargo run -- samples/rotation.ll --qubits 1
```

---

## 🗺 Future Roadmap

- [x] **Classical Control**: Support for `br` instructions (conditional branching) based on measurement results.
- [x] **More Gates**: Support for arbitrary rotation gates (Rx, Ry, Rz).
- [ ] **Python Binding**: Integration with Python via `PyO3`.
- [ ] **WASM Support**: Execution in the browser via WebAssembly.
- [ ] **Visualization**: Add a simple circuit diagram or state-sphere visualization to the output.

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🤝 Contributing
As this is a learning-driven project, feedback, issue reports, and Pull Requests are more than welcome! Let's build the most accessible quantum environment together.
