use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum AppError {
    #[error("Dead")]
    Unknown,
}
