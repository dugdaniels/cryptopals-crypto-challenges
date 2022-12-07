use crate::utils::read_lines;
use std::collections::HashSet;

pub fn detect_aes_ecb_encryption(path: &str, block_size: usize) -> Option<Vec<u8>> {
    let lines = read_lines(path);
    let mut top_count = 0;
    let mut top_line = 0;

    for (i, line) in lines.iter().enumerate() {
        let count = count_identical_blocks(line, block_size);

        if count > top_count {
            top_count = count;
            top_line = i;
        }
    }

    match top_count {
        0 => None,
        _ => Some(lines[top_line].clone()),
    }
}

pub fn count_identical_blocks(bytes: &[u8], block_size: usize) -> usize {
    let blocks: Vec<_> = bytes.chunks_exact(block_size).collect();
    let unique_blocks: HashSet<_> = blocks.iter().cloned().collect();

    blocks.len() - unique_blocks.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_ecb_works() {
        let result = detect_aes_ecb_encryption("src/set_1/files/challenge_8.txt", 16);

        assert!(result.is_some());
    }
}
