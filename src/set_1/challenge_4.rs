use super::challenge_3::{decipher, score};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn detect_single_character_xor(path: &str) -> String {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut text = String::new();
    let mut top_score = 0_f64;

    for line in reader.lines() {
        let line = line.unwrap();
        let score = score(&decipher(&line));

        if score > top_score {
            text = decipher(&line);
            top_score = score;
        }
    }

    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_single_char_xor() {
        let text = detect_single_character_xor("src/set_1/files/challenge_4.txt");
        assert_eq!(text, "Now that the party is jumping\n");
    }
}
