use super::challenge_3::score;
use std::fs;

const MAX_KEYSIZE: usize = 56;

pub fn read_bytes(path: &str) -> Vec<u8> {
    let base64_string = fs::read_to_string(path)
        .map(|res| res.replace('\n', ""))
        .unwrap();

    base64::decode(base64_string).unwrap()
}

pub fn hamming_distance(x_bytes: &[u8], y_bytes: &[u8]) -> u32 {
    let mut distance = 0;

    for (x_byte, y_byte) in x_bytes.iter().zip(y_bytes.iter()) {
        let xor = x_byte ^ y_byte;
        distance += xor.count_ones();
    }

    distance
}

pub fn avg_hamming_distance(bytes: &[u8], key_size: usize) -> f64 {
    let mut block_x;
    let mut block_y;
    let mut i = 0;
    let mut distance = 0;

    loop {
        if (i * key_size) + (2 * key_size) >= bytes.len() {
            break;
        }

        block_x = &bytes[i * key_size..(i + 1) * key_size];
        block_y = &bytes[(i + 1) * key_size..(i + 2) * key_size];

        distance += hamming_distance(block_x, block_y) / (key_size as u32);

        i += 1;
    }

    (distance as f64) / (i as f64)
}

pub fn find_keysize(bytes: &[u8]) -> usize {
    let mut top_distance = f64::MAX;
    let mut keysize = 0;

    (2..=MAX_KEYSIZE).for_each(|i| {
        let distance = avg_hamming_distance(bytes, i);

        if distance < top_distance {
            top_distance = distance;
            keysize = i;
        }
    });

    keysize
}

pub fn break_single_character_xor(bytes: &[u8]) -> u8 {
    let mut key = 0_u8;
    let mut top_score = 0_f64;

    (0..=u8::MAX).for_each(|i| {
        let bytes: Vec<u8> = bytes.iter().map(|&b| b ^ i).collect();

        let message = String::from_utf8_lossy(&bytes);
        let score = score(&message);

        if score > top_score {
            key = i;
            top_score = score;
        }
    });

    key
}

pub fn break_repeating_key_xor(path: &str) -> Vec<u8> {
    let bytes = read_bytes(path);
    let keysize = find_keysize(&bytes);

    let blocks = bytes
        .chunks(keysize)
        .map(|block| block.to_vec())
        .collect::<Vec<_>>();

    let mut key = Vec::new();

    for i in 0..keysize {
        let set = blocks
            .iter()
            .filter(|&block| block.len() > i)
            .map(|block| block[i])
            .collect::<Vec<u8>>();

        key.push(break_single_character_xor(&set));
    }

    key
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hamming_distance_works() {
        let x = b"this is a test";
        let y = b"wokka wokka!!!";

        let distance = hamming_distance(x, y);

        assert_eq!(distance, 37);
    }

    #[test]
    fn crack_works() {
        let key = break_repeating_key_xor("src/set_1/files/challenge_6.txt");
        let key_string = String::from_utf8_lossy(&key).to_string();

        let expected = "Terminator X: Bring the noise";

        assert_eq!(key_string, expected);
    }
}
