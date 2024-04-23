use thiserror::Error;
use std::num::ParseIntError;

#[derive(Error, Debug)]
pub enum ScanError {
    #[error("Invalid input format")]
    InvalidFormat(#[from] ParseIntError),
    #[error("Invalid port range format")]
    InvalidPortRange,
    #[error("unknown scann error")]
    Unknown,
}