pub fn fixex_xor(x: &str, y: &str) -> String {
    let x_bytes = hex::decode(x).unwrap();
    let y_bytes = hex::decode(y).unwrap();

    let mut result = Vec::new();

    for (x_byte, y_byte) in x_bytes.iter().zip(y_bytes.iter()) {
        result.push(x_byte ^ y_byte);
    }

    hex::encode(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_xor() {
        let a = "1c0111001f010100061a024b53535009181c";
        let b = "686974207468652062756c6c277320657965";
        let result = "746865206b696420646f6e277420706c6179";

        assert_eq!(fixex_xor(a, b), result);
    }
}
