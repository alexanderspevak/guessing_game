use crate::constants::ID_LENGTH;
use crate::traits::Packable;
use crate::MessageError;

#[derive(Default, Debug)]
pub struct OpponentSelected {
    pub guessing_player: String,
    pub asking_player: String,
}

impl Packable for OpponentSelected {
    fn pack(&self) -> Vec<u8> {
        let mut result = self.guessing_player.as_bytes().to_vec();
        result.extend_from_slice(self.asking_player.as_bytes());
        result
    }

    fn unpack(&mut self, msg_bytes: &[u8]) -> Result<(), MessageError> {
        self.guessing_player = String::from_utf8(msg_bytes[0..ID_LENGTH as usize].to_vec())
            .map_err(|_| MessageError::BadUnpack("Invalid UTF-8 sequence in player ID"))?;
        self.asking_player = String::from_utf8(msg_bytes[ID_LENGTH as usize..].to_vec())
            .map_err(|_| MessageError::BadUnpack("Invalid UTF-8 sequence in  opponent ID"))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::get_random_id;

    #[test]
    fn pack_unpack_opponent() {
        let player_id = get_random_id();
        let opponent_id = get_random_id();
        assert_eq!(player_id.len(), ID_LENGTH as usize);

        let id_instance = OpponentSelected {
            guessing_player: player_id.clone(),
            asking_player: opponent_id.clone(),
        };

        let bytes = id_instance.pack();
        let mut check_instance = OpponentSelected::default();
        check_instance
            .unpack(&bytes)
            .expect("Unpacking should not fail");

        assert_eq!(check_instance.guessing_player, player_id);
        assert_eq!(check_instance.asking_player, opponent_id);
    }
}
