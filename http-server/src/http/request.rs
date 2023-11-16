use crate::http::request;

use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;
#[derive(Debug)]
pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buf)?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequestLine)?;

        unimplemented!()
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    let iter = request.chars();

    for (i, c) in request.chars().enumerate() {
        println!("c: {} and i: {}", c, i);
        if c == ' ' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    None
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
            Self::InvalidRequestLine => "inv Request",
            Self::InvalidProtocol => "inv Protocol",
            Self::InvalidMethod => "inv Method",
            _ => "unkown error",
        }
    }
}

impl Error for ParseError {}
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

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}
