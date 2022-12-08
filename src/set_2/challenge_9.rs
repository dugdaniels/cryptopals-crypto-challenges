pub fn pad_pkcs7(data: &[u8], block_size: usize) -> Vec<u8> {
    let mut padded = data.to_vec();
    let padding = block_size - (data.len() % block_size);
    padded.extend(vec![padding as u8; padding]);

    padded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pad_pkcs7_works() {
        let data = b"YELLOW SUBMARINE";
        let padded = pad_pkcs7(data, 20);

        let message = String::from_utf8_lossy(&padded);
        let expected = "YELLOW SUBMARINE\x04\x04\x04\x04";

        assert_eq!(message, expected);
    }
}
