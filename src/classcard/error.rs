use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    ReadError,
    GetDataError,
    InvalidCmd(String),

}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReadError => write!(f, "Failed to read from socket"),
            Self::GetDataError => write!(f, "Failed to get data from socket"),
            Self::InvalidCmd(s) => write!(f, "Invalid cmd: {}", s),
        }
    }
}

impl std::error::Error for Error {}