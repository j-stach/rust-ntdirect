
use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum NTDirectError {
    ConfigurationError(String),
    ConnectionError(String),
    OrderError(String),
    CommandError(String),
}
impl Error for NTDirectError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            _ => None,
        }
    }
}
impl Display for NTDirectError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            NTDirectError::ConfigurationError(str) => write!(f, "ConfigurationError: {} has failed", str),
            NTDirectError::ConnectionError(str) => write!(f, "ConnectionError: {}", str), 
            NTDirectError::OrderError(str) => write!(f, "OrderError: {} order has failed", str),
            NTDirectError::CommandError(str) => write!(f, "Command \"{}\" has failed", str),
        }
    }
}