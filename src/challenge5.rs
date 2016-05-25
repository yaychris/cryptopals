use challenge1::hex_to_bytes;
use rustc_serialize::hex::ToHex;
use challenge2::xor;

fn repeat_bytes(bytes: &[u8], length: usize) -> Vec<u8> {
    bytes.into_iter().cloned().cycle().take(length).collect()
}

pub fn rkxor_encrypt(key: &str, plain: &str) -> String {
    let plain_bytes = plain.as_bytes();

    // Pad key to length
    let padded_key = repeat_bytes(key.as_bytes(), plain_bytes.len());

    // Xor padded key and plain text
    let cipher = xor(&padded_key, plain_bytes).unwrap();

    // Convert cipher to hex
    cipher.as_slice().to_hex()
}

pub fn rkxor_decrypt(key: &str, cipher: &str) -> String {
    let cipher_bytes = hex_to_bytes(cipher).unwrap();

    // Pad key to length
    let padded_key = repeat_bytes(key.as_bytes(), cipher_bytes.len());

    // Xor padded key and cipher text
    let plain = xor(&padded_key, &cipher_bytes).unwrap();

    // Convert plain to UTF-8 string
    String::from_utf8(plain).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{rkxor_encrypt, rkxor_decrypt};

    const KEY: &'static str = "ICE";

    const PLAIN: &'static str = "Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal";

    const CIPHER: &'static str = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

    #[test]
    fn repeating_key_xor_encrypt() {
        assert_eq!(rkxor_encrypt(KEY, PLAIN), CIPHER);
    }

    #[test]
    fn repeating_key_xor_decrypt() {
        assert_eq!(rkxor_decrypt(KEY, CIPHER), PLAIN);
    }
}
