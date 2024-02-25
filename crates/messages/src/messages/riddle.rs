use crate::constants::ID_LENGTH;
use crate::helpers::get_string_slice_length;
use crate::traits::Packable;
use crate::MessageError;

#[derive(Default, Clone)]
pub struct Riddle {
    pub sender: String,
    pub asking_player: String,
    pub guessing_player: String,
    pub message: String,
    pub hint: Option<String>,
}

impl Packable for Riddle {
    fn pack(&self) -> Vec<u8> {
        let question_length = get_string_slice_length(&self.message);
        let hint_length = self.hint.as_ref().map_or(0, |h| get_string_slice_length(h));
        let mut packed = vec![];

        packed.extend_from_slice(self.sender.as_bytes());
        packed.extend_from_slice(self.asking_player.as_bytes());
        packed.extend_from_slice(self.guessing_player.as_bytes());
        packed.push(question_length);
        packed.extend_from_slice(self.message.as_bytes());
        packed.push(hint_length);
        packed.extend_from_slice(self.hint.as_ref().map_or(&[], |h| h.as_bytes()));

        packed
    }

    fn unpack(&mut self, msg_bytes: &[u8]) -> Result<(), MessageError> {
        let mut offset = 0;
        let id_end = offset + ID_LENGTH as usize;
        let sender_bytes = msg_bytes
            .get(offset..id_end)
            .ok_or(MessageError::BadUnpack(
                "Invalid message format: sending player id missing",
            ))?;
        self.sender = String::from_utf8(sender_bytes.to_vec())
            .map_err(|_| MessageError::BadUnpack("Invalid UTF-8 sequence in sender id"))?;

        offset = id_end;
        let id_end = offset + ID_LENGTH as usize;
        let asking_player_bytes = msg_bytes
            .get(offset..id_end)
            .ok_or(MessageError::BadUnpack(
                "Invalid message format: asking player ID missing",
            ))?;
        self.asking_player = String::from_utf8(asking_player_bytes.to_vec()).map_err(|_| {
            MessageError::BadUnpack("Invalid UTF-8 sequence in from asking player id")
        })?;

        offset = id_end;
        let id_end = offset + ID_LENGTH as usize;
        let guessing_player_bytes =
            msg_bytes
                .get(offset..id_end)
                .ok_or(MessageError::BadUnpack(
                    "Invalid message format: guessing player id missing",
                ))?;
        self.guessing_player = String::from_utf8(guessing_player_bytes.to_vec()).map_err(|_| {
            MessageError::BadUnpack("Invalid UTF-8 sequence in from guessing player id")
        })?;

        offset = id_end;
        let message_length = *msg_bytes.get(offset).ok_or(MessageError::BadUnpack(
            "Invalid message format: Message length missing",
        ))? as usize;
        offset += 1;

        let message_end = offset + message_length;
        let message_bytes = msg_bytes
            .get(offset..message_end)
            .ok_or(MessageError::BadUnpack(
                "Invalid message format: Message missing",
            ))?;
        self.message = String::from_utf8(message_bytes.to_vec())
            .map_err(|_| MessageError::BadUnpack("Invalid UTF-8 sequence in message"))?;
        offset = message_end;

        let hint_length = *msg_bytes.get(offset).ok_or(MessageError::BadUnpack(
            "Invalid message format: Hint length missing",
        ))? as usize;
        offset += 1;

        if hint_length > 0 {
            let hint_end = offset + hint_length;
            let hint_bytes = msg_bytes
                .get(offset..hint_end)
                .ok_or(MessageError::BadUnpack(
                    "Invalid message format: Hint missing",
                ))?;
            self.hint = Some(
                String::from_utf8(hint_bytes.to_vec())
                    .map_err(|_| MessageError::BadUnpack("Invalid UTF-8 sequence in hint"))?,
            );
        } else {
            self.hint = None;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::helpers::get_random_id;

    #[test]
    fn pack_unpack_riddle() {
        let sender = get_random_id();
        let asking_player = get_random_id();
        let guessing_player = get_random_id();
        let message = String::from("What is meaning of life?");
        let hint = String::from("Galaxy");
        let riddle_instance = Riddle {
            sender,
            asking_player,
            guessing_player,
            message,
            hint: Some(hint.clone()),
        };

        let bytes = riddle_instance.pack();
        let mut check_instance = Riddle::default();
        check_instance
            .unpack(&bytes)
            .expect("Unpacking should not fail");

        assert_eq!(check_instance.sender, riddle_instance.sender);
        assert_eq!(check_instance.asking_player, riddle_instance.asking_player);
        assert_eq!(
            check_instance.guessing_player,
            riddle_instance.guessing_player
        );
        assert_eq!(check_instance.message, riddle_instance.message);
        assert_eq!(check_instance.hint, Some(hint));
    }

    #[test]
    fn pack_unpack_riddle_with_no_hint() {
        let sender = get_random_id();
        let asking_player = get_random_id();
        let guessing_player = get_random_id();
        let message = String::from("What is meaning of life?");

        let riddle_instance = Riddle {
            sender,
            asking_player,
            guessing_player,
            message,
            hint: None,
        };

        let bytes = riddle_instance.pack();

        let mut check_instance = Riddle::default();

        check_instance
            .unpack(&bytes)
            .expect("Unpacking should not fail");

        assert_eq!(check_instance.sender, riddle_instance.sender);
        assert_eq!(check_instance.asking_player, riddle_instance.asking_player);
        assert_eq!(
            check_instance.guessing_player,
            riddle_instance.guessing_player
        );
        assert_eq!(check_instance.message, riddle_instance.message);
        assert_eq!(check_instance.hint, None);
    }
}
