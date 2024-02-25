use crate::constants::ID_LENGTH;
use crate::traits::Packable;
use crate::MessageError;

#[derive(Default, Debug)]
pub struct PlayerId {
    pub player_id: String,
}

impl Packable for PlayerId {
    fn pack(&self) -> Vec<u8> {
        let mut packed = vec![];
        packed.extend_from_slice(self.player_id.as_bytes());

        packed
    }
    fn unpack(&mut self, msg_bytes: &[u8]) -> Result<(), MessageError> {
        let receiver_id_bytes =
            msg_bytes
                .get(0..ID_LENGTH as usize)
                .ok_or(MessageError::BadUnpack(
                    "Invalid message format: Receiver ID missing",
                ))?;
        self.player_id = String::from_utf8(receiver_id_bytes.to_vec())
            .map_err(|_| MessageError::BadUnpack("Invalid UTF-8 sequence in receiver ID"))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::get_random_id;

    use super::*;

    #[test]
    fn pack_unpack_player_id() {
        let player_id = get_random_id();
        let player_id_instance = PlayerId {
            player_id: player_id.clone(),
        };

        let bytes = player_id_instance.pack();
        let mut check_instance = PlayerId::default();
        check_instance
            .unpack(&bytes)
            .expect("Unpacking should not fail");

        assert_eq!(player_id, check_instance.player_id);
    }
}
