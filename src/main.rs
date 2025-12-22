mod error;
mod memory;
mod simulator;
mod parser;
mod qis_bridge;

use clap::Parser;
use crate::parser::QirParser;
use crate::simulator::Simulator;
use crate::memory::QirMemory;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 入力するQIRファイル (.ll または .bc)
    file: String,

    /// 使用する量子ビット数
    #[arg(short, long, default_value_t = 10)]
    qubits: usize,
}

fn main() -> error::Result<()> {
    let args = Args::parse();

    println!("Loading QIR file: {}", args.file);
    let parser = QirParser::from_file(&args.file)?;

    let mut simulator = Simulator::new(args.qubits);
    let mut memory = QirMemory::new();
    let mut bridge = qis_bridge::QisBridge::new(&mut simulator, &mut memory);
    
    println!("Simulator initialized with {} qubits.", args.qubits);
    println!("Starting execution...");

    parser.run_interpreter(&mut bridge)?;

    println!("Execution finished.");
    simulator.print_probabilities();
    Ok(())
}
