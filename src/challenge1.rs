use rustc_serialize::hex::{FromHex, ToHex, FromHexError};
use rustc_serialize::base64::{FromBase64, ToBase64, FromBase64Error, STANDARD};


pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, FromHexError> {
    hex.from_hex()
}

pub fn hex_to_base64(hex: &str) -> String {
    hex_to_bytes(hex).unwrap().to_base64(STANDARD)
}

pub fn base64_to_bytes(base64: &str) -> Result<Vec<u8>, FromBase64Error> {
    base64.from_base64()
}

pub fn base64_to_hex(base64: &str) -> String {
    base64_to_bytes(base64).unwrap().to_hex()
}


#[cfg(test)]
mod tests {
    use super::*;

    const HEX: &'static str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";

    const BASE64: &'static str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

    #[test]
    fn convert_hex_to_base64() {
        assert_eq!(BASE64, hex_to_base64(HEX));
    }

    #[test]
    fn convert_base64_to_hex() {
        assert_eq!(HEX, base64_to_hex(BASE64));
    }
}
