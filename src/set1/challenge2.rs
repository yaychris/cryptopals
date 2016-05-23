use std::fmt;
use self::XorError::*;

#[derive(Clone, Copy)]
pub enum XorError {
    DifferentLengths(usize, usize)
}

impl fmt::Debug for XorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let XorError::DifferentLengths(l, r) = *self;

        write!(f, "Lengths do not match: left = {}, right = {}", l, r)
    }
}

pub fn xor(l: &Vec<u8>, r: &Vec<u8>) -> Result<Vec<u8>, XorError> {
    if l.len() != r.len() {
        return Err(DifferentLengths(l.len(), r.len()));
    }

    let result = l.iter().zip(r.iter()).map(|(l, r)| l ^ r).collect();

    Ok(result)
}


#[cfg(test)]
mod tests {
    use super::*;
    use set1::challenge1::hex_to_bytes;
    use rustc_serialize::hex::ToHex;

    const INPUT1: &'static str = "1c0111001f010100061a024b53535009181c";
    const INPUT2: &'static str = "686974207468652062756c6c277320657965";
    const OUTPUT: &'static str = "746865206b696420646f6e277420706c6179";

    #[test]
    fn xor_ok_result() {
        let bytes1 = hex_to_bytes(INPUT1).unwrap();
        let bytes2 = hex_to_bytes(INPUT2).unwrap();

        let bytes = xor(&bytes1, &bytes2).unwrap();

        assert_eq!(bytes.to_hex(), OUTPUT);
    }

    #[test]
    fn xor_different_lengths() {
        let bytes1 = &"ab".as_bytes().to_vec();
        let bytes2 = &"cdef".as_bytes().to_vec();

        let result = xor(&bytes1, &bytes2);

        assert!(result.is_err());
    }
}
