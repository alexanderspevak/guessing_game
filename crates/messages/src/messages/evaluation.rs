use crate::{traits::Packable, MessageError};

#[derive(Default)]
pub struct Evaluation {
    pub hint: Option<String>,
    pub guessed: bool,
}

impl Packable for Evaluation {
    fn pack(&self) -> Vec<u8> {
        let mut packed = Vec::new();
        packed.push(self.guessed as u8);

        if let Some(hint) = &self.hint {
            let hint_bytes = hint.as_bytes();
            packed.extend_from_slice(hint_bytes);
        }

        packed
    }

    fn unpack(&mut self, msg_bytes: &[u8]) -> Result<(), MessageError> {
        if msg_bytes.is_empty() {
            return Err(MessageError::BadUnpack(
                "Invalid message format: data missing",
            ));
        }

        self.guessed = msg_bytes[0] != 0;

        let hint_slice = &msg_bytes[1..];

        if hint_slice.is_empty() {
            self.hint = None;
            return Ok(());
        }

        let hint = String::from_utf8(hint_slice.to_vec())
            .map_err(|_| MessageError::BadUnpack("Invalid hint bytes"))?;
        self.hint = Some(hint);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pack_unpack_evaluation_with_hint() {
        let hint_value = Some(String::from("This is a hint."));
        let guessed_value = true;

        let evaluation_instance = Evaluation {
            hint: hint_value.clone(),
            guessed: guessed_value,
        };

        let bytes = evaluation_instance.pack();
        let mut check_instance = Evaluation::default();
        check_instance
            .unpack(&bytes)
            .expect("Unpacking should not fail");

        assert_eq!(check_instance.hint, hint_value);
        assert_eq!(check_instance.guessed, guessed_value);
    }

    #[test]
    fn pack_unpack_evaluation_without_hint() {
        let hint_value = None;
        let guessed_value = false;

        let evaluation_instance = Evaluation {
            hint: hint_value,
            guessed: guessed_value,
        };

        let bytes = evaluation_instance.pack();
        let mut check_instance = Evaluation::default();
        check_instance
            .unpack(&bytes)
            .expect("Unpacking should not fail");

        assert_eq!(check_instance.hint, None);
        assert_eq!(check_instance.guessed, guessed_value);
    }
}
