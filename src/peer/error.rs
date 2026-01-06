use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum HandshakeError {
    InvalidLength,
    InvalidProtocolLength(u8),
    InvalidProtocolString,
    InfoHashMismatch,
    SelfConnection,
}

impl fmt::Display for HandshakeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HandshakeError::InvalidLength => write!(f, "Invalid input length"),
            HandshakeError::InvalidProtocolLength(val) => {
                write!(f, "Invalid Protocol Length: {}", val)
            }
            HandshakeError::InvalidProtocolString => write!(f, "Invalid Protocol String"),
            HandshakeError::InfoHashMismatch => write!(f, "Info Hash Mismatch"),
            HandshakeError::SelfConnection => {
                write!(f, "Self Connection: Tried to connect to self")
            }
        }
    }
}

impl Error for HandshakeError {}

#[derive(Debug)]
pub enum PeerHandshakeError {
    HandshakeError(HandshakeError),
    IOError(std::io::Error),
}

impl fmt::Display for PeerHandshakeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::HandshakeError(h) => write!(f, "{}", h),
            Self::IOError(i) => write!(f, "{}", i),
        }
    }
}

impl Error for PeerHandshakeError {}

impl From<std::io::Error> for PeerHandshakeError {
    fn from(err: std::io::Error) -> Self {
        PeerHandshakeError::IOError(err)
    }
}

impl From<HandshakeError> for PeerHandshakeError {
    fn from(err: HandshakeError) -> Self {
        PeerHandshakeError::HandshakeError(err)
    }
}
