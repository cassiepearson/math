//! Shift Cipher Implementation

/// Implements a modernized shift cipher or Caesar cipher for String
///
/// Valid key range is u8 (0-255)
/// Spaces and special characters are allowed and enciphered as characters
/// The result should never return an error since the key is a valid u8
pub trait ShiftCipher {
    fn encipher(self, key: u8) -> String;
    fn decipher(self, key: u8) -> String;
}

impl ShiftCipher for String {
    // Since the bytes are always within the range [0, 255], the error is never
    // returned, so the unwrap is safe.
    fn encipher(self, key: u8) -> String {
        String::from_utf8(
            self.into_bytes()
                .into_iter()
                .map(|char| char.wrapping_add(key))
                .collect::<Vec<u8>>(),
        )
        .unwrap()
    }

    fn decipher(self, key: u8) -> String {
        // Since the bytes are always within the range [0, 255], the error is never
        // returned, so the unwrap is safe.
        String::from_utf8(
            self.into_bytes()
                .into_iter()
                .map(|char| char.wrapping_sub(key))
                .collect::<Vec<u8>>(),
        )
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0, "abcdef".to_string(), "abcdef".to_string())]
    #[case(1, "abcdef".to_string(), "bcdefg".to_string())]
    #[case(2, "abcdef".to_string(), "cdefgh".to_string())]
    #[case(252, "abcdef".to_string(), "]^_`ab".to_string())]
    fn shift_encipher_test(#[case] key: u8, #[case] message: String, #[case] expected: String) {
        assert_eq!(expected, message.encipher(key))
    }

    #[rstest]
    #[case(0, "abcdef".to_string(), "abcdef".to_string())]
    #[case(1, "bcdefg".to_string(), "abcdef".to_string())]
    #[case(2, "cdefgh".to_string(), "abcdef".to_string())]
    #[case(252, "]^_`ab".to_string(), "abcdef".to_string())]
    fn shift_decipher_test(#[case] key: u8, #[case] message: String, #[case] expected: String) {
        assert_eq!(expected, message.decipher(key))
    }
}
