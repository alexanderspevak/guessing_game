use crate::constants;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::iter;

pub fn get_string_slice_length(str: &str) -> u8 {
    if str.len() > 255 {
        panic!("string length can be max 255 characters");
    }
    str.len() as u8
}

pub fn get_random_id() -> String {
    let mut rng = thread_rng();
    iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(constants::ID_LENGTH as usize)
        .collect()
}

pub fn split_u16(value: u16) -> (u8, u8) {
    let high_byte = (value >> 8) as u8;
    let low_byte = (value & 0xFF) as u8;
    (high_byte, low_byte)
}

pub fn merge_u8(high_byte: u8, low_byte: u8) -> u16 {
    ((high_byte as u16) << 8) | (low_byte as u16)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_merge_split_u8_to_u16() {
        let (a, b) = split_u16(645);
        let value = merge_u8(a, b);
        assert_eq!(value, 645);
    }
}
