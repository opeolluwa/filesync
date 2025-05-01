use thiserror::Error;

#[derive(Error, Debug)]
pub enum StartupError {
    #[error("Process failed due to {0}")]
    ProcessFailed(String),
}
