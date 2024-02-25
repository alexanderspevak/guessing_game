use crate::{traits::Packable, MessageError};

#[derive(Default)]
pub struct Password {
    pub password: String,
}

impl Packable for Password {
    fn pack(&self) -> Vec<u8> {
        let password_bytes = self.password.as_bytes().to_vec();
        password_bytes
    }

    fn unpack(&mut self, msg_bytes: &[u8]) -> Result<(), MessageError> {
        self.password = String::from_utf8(msg_bytes.to_vec())
            .map_err(|_| MessageError::BadUnpack("Invalid UTF-8 sequence in password"))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pack_unpack_pasword() {
        let id_instance = Password {
            password: "12345".into(),
        };

        let bytes = id_instance.pack();
        let mut check_instance = Password::default();
        check_instance
            .unpack(&bytes)
            .expect("Unpacking should not fail");

        assert_eq!(&check_instance.password, "12345");
    }
}
