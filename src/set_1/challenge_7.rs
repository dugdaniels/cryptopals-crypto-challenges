use crate::utils::read_bytes;
use aes::cipher::{generic_array::GenericArray, BlockDecrypt, KeyInit};
use aes::Aes128;

pub fn decrypt(path: &str, key: &[u8]) -> Vec<u8> {
    let bytes = read_bytes(path);
    let cipher = Aes128::new(GenericArray::from_slice(key));

    bytes
        .chunks_exact(key.len())
        .flat_map(|chunk| {
            let mut block = GenericArray::clone_from_slice(chunk);
            cipher.decrypt_block(&mut block);
            block.to_vec()
        })
        .collect::<Vec<u8>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decrypt_works() {
        let key = b"YELLOW SUBMARINE";
        let message = decrypt("src/set_1/files/challenge_7.txt", key);

        let message_string = String::from_utf8_lossy(&message).to_string();

        assert!(message_string.contains("Play that funky music"));
    }
}
