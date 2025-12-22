use thiserror::Error;

#[derive(Error, Debug)]
pub enum QirError {
    #[error("パースエラー: {0}")]
    ParseError(String),

    #[error("量子命令エラー: {0}")]
    InstructionError(String),

    #[error("シミュレータエラー: {0}")]
    SimulatorError(String),

    #[error("IOエラー: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, QirError>;
