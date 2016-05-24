#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::fs::File;
    use std::io::Read;

    use challenge3::guess_key;

    #[test]
    fn find_string() {
        let file = read_file("files/challenge4.txt");

        let guess = file.lines()
                        .filter_map(guess_key)
                        .max_by_key(|x| x.etaoin_score)
                        .unwrap();

        assert_eq!(guess.key,          "5");
        assert_eq!(guess.etaoin_score, 11);
        assert_eq!(guess.plain_text,   "Now that the party is jumping\n");
    }

    fn read_file(file: &str) -> String {
        let path = Path::new(file);

        let mut file = File::open(path).unwrap();
        let mut contents = String::new();

        let _ = file.read_to_string(&mut contents);

        contents
    }
}
