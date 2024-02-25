use crate::{Message, MessageError};

pub trait Packable {
    fn pack(&self) -> Vec<u8>;
    fn unpack(&mut self, msg_bytes: &[u8]) -> Result<(), MessageError>;
}

pub trait Streamable: Send {
    fn read(&mut self) -> Result<Message, MessageError>;
    fn write(&mut self, message: &Message) -> Result<(), MessageError>;
    fn shutdown(&mut self) -> Result<(), &'static str>;
}
