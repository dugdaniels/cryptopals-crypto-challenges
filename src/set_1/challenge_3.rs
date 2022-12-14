const LETTER_FREQUENCY: [f64; 27] = [
    0.08167, 0.01492, 0.02782, 0.04253, 0.12702, 0.02228, 0.02015, // a-g
    0.06094, 0.06966, 0.00153, 0.00772, 0.04025, 0.02406, 0.06749, // h-n
    0.07507, 0.01929, 0.00095, 0.05987, 0.06327, 0.09056, 0.02758, // o-u
    0.00978, 0.02360, 0.00150, 0.01974, 0.00074, 0.19181, // v-z & space
];

pub fn decipher(cipher: &str) -> String {
    let cipher_bytes = hex::decode(cipher).unwrap();

    let mut message = String::new();
    let mut top_score = 0_f64;

    (0..=u8::MAX).for_each(|key| {
        let text_bytes: Vec<u8> = cipher_bytes.iter().map(|&b| b ^ key).collect();

        let text = String::from_utf8_lossy(&text_bytes);
        let score = score(&text);

        if score > top_score {
            message = String::from(text);
            top_score = score;
        }
    });

    message
}

pub fn score(message: &str) -> f64 {
    message.chars().fold(0.0, |score, c| {
        let index = match c {
            'a'..='z' => (c as u8 - b'a') as usize,
            'A'..='Z' => (c as u8 - b'A') as usize,
            ' ' => 26,
            _ => return score,
        };

        score + LETTER_FREQUENCY[index]
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c3() {
        let cipher = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        assert_eq!(decipher(cipher), "Cooking MC's like a pound of bacon");
    }
}
