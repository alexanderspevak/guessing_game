use crate::ConnectionType;
use std::fmt;
use std::fmt::{Debug, Display};

pub enum MessageError {
    BadUnpack(&'static str),
    EmptyRead,
    InvalidRead(ConnectionType),
    InvalidWrite(ConnectionType),
}

impl Display for MessageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::BadUnpack(message) => write!(f, "Bad Unpack: {}", message),
            Self::EmptyRead => write!(f, ""),
            Self::InvalidRead(connection_type) => match connection_type {
                ConnectionType::Tcp => write!(f, "Could not read data from TCP stream."),
                ConnectionType::UnixSocket => write!(f, "Could not read data from unix socket."),
            },
            Self::InvalidWrite(connection_type) => match connection_type {
                ConnectionType::Tcp => write!(f, "Could not write data to TCP stream."),
                ConnectionType::UnixSocket => write!(f, "Could not write data to unix socket."),
            },
        }
    }
}

impl Debug for MessageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
