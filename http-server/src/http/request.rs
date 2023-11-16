use super::method::Method;
use std::error::Error;
use std::fmt::{write, Debug, Display, Formatter, Result as FmtResult};
use std::{convert::TryFrom, error};
#[derive(Debug)]
pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}

// Custom type of Err
pub enum ParseError {
    InvalidRequestLine,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidEncoding => "inv request",
            Self::InvalidEncoding => "inv encoding",
            Self::InvalidProtocol => "inv Protocol",
            Self::InvalidMethod => "inv Method",
            _ => "unkown error",
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}
