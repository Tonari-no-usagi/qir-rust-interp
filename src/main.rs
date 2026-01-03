use clap::Parser as ClapParser;
use qir_rust_interp::parser::QirParser;
use qir_rust_interp::simulator::Simulator;
use qir_rust_interp::memory::QirMemory;
use qir_rust_interp::qis_bridge::QisBridge;
use qir_rust_interp::error;

#[derive(ClapParser, Debug)]
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
    let mut bridge = QisBridge::new(&mut simulator, &mut memory);
    
    println!("Simulator initialized with {} qubits.", args.qubits);
    println!("Starting execution...");

    parser.run_interpreter(&mut bridge)?;

    println!("Execution finished.");
    simulator.print_probabilities();
    Ok(())
}
