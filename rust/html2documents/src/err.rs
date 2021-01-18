use std::io;
use std::fmt;

#[derive(Debug)]
pub enum HTML2DocumentsError {
    IOError(io::Error),
    ParseError(io::Error)
} 

impl fmt::Display for HTML2DocumentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HTML2DocumentsError::IOError(e) => { write!(f, "{}", e) },
            HTML2DocumentsError::ParseError(e) => { write!(f, "{}", e) },
        }
    }
}

impl HTML2DocumentsError {
    pub fn new_io_error(msg: &str) -> HTML2DocumentsError {
        HTML2DocumentsError::IOError(io::Error::new(io::ErrorKind::Other, msg))
    }
    pub fn new_parse_error(msg: &str) -> HTML2DocumentsError {
        HTML2DocumentsError::ParseError(io::Error::new(io::ErrorKind::Other, msg))
    }
}

pub type Result<T> = std::result::Result<T, HTML2DocumentsError>;
