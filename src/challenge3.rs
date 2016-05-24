use challenge2::xor;
use rustc_serialize::hex::FromHex;

fn pad_byte_to_length(byte: u8, length: usize) -> Vec<u8> {
    vec![byte].into_iter().cycle().take(length).collect()
}

pub fn calculate_score(input: &str) -> usize {
    let mut score = 0;

    for c in input.chars() {
        match c {
            'e' | 't' | 'a' | 'o' | 'i' | 'n' => score += 1,
            _   => ()
        }
    }

    score
}

pub struct KeyGuess {
    pub key:          String,
    pub cipher_text:  String,
    pub plain_text:   String,
    pub etaoin_score: usize,
}

pub fn guess_key(input_str: &str) -> KeyGuess {
    // Convert input to bytes
    let input = input_str.from_hex().unwrap();

    let mut highest_scoring_byte = 0u8;
    let mut highest_score        = 0;
    let mut plain_text          = "".to_string();

    // For every byte from 0 to 127
    for byte in 0u8..128u8 {
        // Pad key to same length as input
        let key = pad_byte_to_length(byte, input.len());

        // Xor key against input
        let decrypted_bytes = xor(&key, &input).unwrap();
        let decrypted       = String::from_utf8(decrypted_bytes).unwrap();

        // Calculate score
        let score = calculate_score(&decrypted);

        if score > highest_score {
            highest_score        = score;
            highest_scoring_byte = byte;
            plain_text           = decrypted.clone();
        }
    }

    KeyGuess {
        key:          String::from_utf8(vec![highest_scoring_byte]).unwrap(),
        plain_text:   plain_text,
        cipher_text:  input_str.into(),
        etaoin_score: highest_score
    }
}

#[cfg(test)]
mod tests {
    use super::guess_key;

    const INPUT: &'static str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

    #[test]
    fn decrypt() {
        let guess = guess_key(&INPUT);

        assert_eq!(guess.key, "X");
        assert_eq!(guess.plain_text, "Cooking MC's like a pound of bacon");
    }
}
