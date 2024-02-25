use crate::helpers::merge_u8;
use crate::pack;
use crate::traits::Streamable;
use crate::unpack_without_headers;
use crate::ConnectionType;
use crate::Message;
use crate::MessageError;
use crate::{HEADERS_LEN, MESSAGE_PREFIX};
use std::{
    io::prelude::*,
    net::{Shutdown, TcpStream},
};

pub struct TcpMessageStream {
    pub stream: TcpStream,
}

impl Streamable for TcpMessageStream {
    fn read(&mut self) -> Result<Message, MessageError> {
        let mut meta_container = [0_u8; HEADERS_LEN];
        self.stream.read_exact(&mut meta_container).map_err(|e| {
            if e.kind() == std::io::ErrorKind::WouldBlock {
                return MessageError::EmptyRead;
            }
            MessageError::InvalidRead(ConnectionType::Tcp)
        })?;

        let size_slice = &meta_container[MESSAGE_PREFIX.len()..];
        let big_endian = size_slice[0];
        let small_endian = size_slice[1];
        let message_size = merge_u8(big_endian, small_endian) as usize;
        let mut message_container = vec![0u8; message_size];
        self.stream
            .read_exact(&mut message_container)
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::WouldBlock {
                    return MessageError::EmptyRead;
                }
                MessageError::InvalidRead(ConnectionType::Tcp)
            })?;

        unpack_without_headers(&message_container)
    }

    fn write(&mut self, message: &Message) -> Result<(), MessageError> {
        let bytes = pack(message);
        self.stream
            .write_all(&bytes)
            .map_err(|_| MessageError::InvalidWrite(ConnectionType::Tcp))
    }

    fn shutdown(&mut self) -> Result<(), &'static str> {
        self.stream
            .shutdown(Shutdown::Both)
            .map_err(|_| "Unsuccessful TCP shutdown")
    }
}
