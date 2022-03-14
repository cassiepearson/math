//! Shift Cipher Implementation
use std::string::FromUtf8Error;

/// Implements a modernized shift cipher or Caesar cipher for String
///
/// Valid key range is u8 (0-255)
/// Spaces and special characters are allowed and enciphered as characters
pub trait ShiftCipher {
    fn encipher(self, key: u8) -> Result<String, FromUtf8Error>;
    fn decipher(self, key: u8) -> Result<String, FromUtf8Error>;
}

impl ShiftCipher for String {
    fn encipher(self, key: u8) -> Result<String, FromUtf8Error> {
        String::from_utf8(
            self.into_bytes()
                .into_iter()
                .map(|char| (char + key) % std::u8::MAX)
                .collect::<Vec<u8>>(),
        )
    }

    fn decipher(self, key: u8) -> Result<String, FromUtf8Error> {
        String::from_utf8(
            self.into_bytes()
                .into_iter()
                .map(|char| (char - key) % std::u8::MAX)
                .collect::<Vec<u8>>(),
        )
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
        assert_eq!(expected, message.encipher(key).unwrap())
    }

    #[rstest]
    #[case(0, "abcdef".to_string(), "abcdef".to_string())]
    #[case(1, "bcdefg".to_string(), "abcdef".to_string())]
    #[case(2, "cdefgh".to_string(), "abcdef".to_string())]
    #[case(252, "]^_`ab".to_string(), "abcdef".to_string())]
    fn shift_decipher_test(#[case] key: u8, #[case] message: String, #[case] expected: String) {
        assert_eq!(expected, message.decipher(key).unwrap())
    }
}
