use std::io;

#[derive(Debug)]
pub enum HTML2DocumentsError {
    IOError(io::Error),
} 

impl HTML2DocumentsError {
    pub fn new(msg: &str) -> HTML2DocumentsError {
        HTML2DocumentsError::IOError(io::Error::new(io::ErrorKind::Other, msg))
    }
}

pub type Result<T> = std::result::Result<T, HTML2DocumentsError>;
