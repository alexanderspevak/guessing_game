use crate::constants::ID_LENGTH;
use crate::traits::Packable;
use crate::MessageError;

#[derive(Default, Debug)]
pub struct PlayerList {
    pub opponent_ids: Vec<String>,
}

impl Packable for PlayerList {
    fn pack(&self) -> Vec<u8> {
        let mut packed = vec![];
        let count = self.opponent_ids.len() as u8;
        packed.push(count);
        for id in &self.opponent_ids {
            packed.extend_from_slice(id.as_bytes());
        }

        packed
    }

    fn unpack(&mut self, msg_bytes: &[u8]) -> Result<(), MessageError> {
        let mut offset = 0;

        let count = *msg_bytes.get(offset).ok_or(MessageError::BadUnpack(
            "Invalid message format: opponent count missing",
        ))? as usize;
        offset += 1;

        self.opponent_ids.clear();

        for _ in 0..count {
            let id_end = offset + ID_LENGTH as usize;
            let id_bytes = msg_bytes
                .get(offset..id_end)
                .ok_or(MessageError::BadUnpack(
                    "Invalid message format: ID missing",
                ))?;
            self.opponent_ids.push(
                String::from_utf8(id_bytes.to_vec()).map_err(|_| {
                    MessageError::BadUnpack("Invalid UTF-8 sequence in opponent ID")
                })?,
            );
            offset = id_end;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::get_random_id;

    #[test]
    fn pack_unpack_player_list() {
        let opponent_ids = vec![get_random_id(), get_random_id()];

        let oponents_instance = PlayerList {
            opponent_ids: opponent_ids.clone(),
        };

        let bytes = oponents_instance.pack();
        let mut check_instance = PlayerList::default();
        check_instance
            .unpack(&bytes)
            .expect("Unpacking should not fail");

        assert_eq!(check_instance.opponent_ids, opponent_ids);
    }

    #[test]
    fn pack_unpack_player_list_none_id() {
        let opponent_ids = vec![get_random_id(), get_random_id()];

        let oponents_instance = PlayerList {
            opponent_ids: opponent_ids.clone(),
        };

        let bytes = oponents_instance.pack();
        let mut check_instance = PlayerList::default();
        check_instance
            .unpack(&bytes)
            .expect("Unpacking should not fail");

        assert_eq!(check_instance.opponent_ids, opponent_ids);
    }
}
