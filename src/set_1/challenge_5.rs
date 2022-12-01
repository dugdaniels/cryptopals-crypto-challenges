pub fn repeating_key_xor(text: &str, key: &str) -> String {
    let mut result = Vec::new();
    let key_bytes = key.as_bytes();

    for (i, byte) in text.bytes().enumerate() {
        result.push(byte ^ key_bytes[i % key.len()]);
    }

    hex::encode(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeating_key_xor() {
        let text = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let key = "ICE";

        let result = repeating_key_xor(text, key);
        let expected = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

        assert_eq!(result, expected);
    }
}
