use super::challenge_9::pad_pkcs7;
use aes::cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit};
use aes::Aes128;

pub fn encrypt(message: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    let message_bytes = pad_pkcs7(message, key.len());
    let cipher = Aes128::new(GenericArray::from_slice(key));
    let iv = iv.to_vec();

    let mut encrypted_blocks: Vec<Vec<u8>> = Vec::new();

    for block in message_bytes.chunks_exact(key.len()) {
        let previous_block = encrypted_blocks.last().unwrap_or(&iv);
        let xor_bytes = xor_bytes(block, previous_block);
        let mut xor_block = GenericArray::clone_from_slice(&xor_bytes);

        cipher.encrypt_block(&mut xor_block);
        encrypted_blocks.push(xor_block.to_vec());
    }

    encrypted_blocks.into_iter().flatten().collect::<Vec<u8>>()
}

pub fn decrypt(ciphertext: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    let cipher = Aes128::new(GenericArray::from_slice(key));
    let iv = iv.to_vec();

    let encrypted_blocks = ciphertext
        .chunks(key.len())
        .map(|block| block.to_vec())
        .collect::<Vec<_>>();

    let mut decrypted_blocks: Vec<Vec<u8>> = Vec::new();

    for (i, block) in encrypted_blocks.iter().enumerate() {
        let mut block = GenericArray::clone_from_slice(block);
        let previous_block = match i {
            0 => &iv,
            _ => &encrypted_blocks[i - 1],
        };

        cipher.decrypt_block(&mut block);
        decrypted_blocks.push(xor_bytes(&block, previous_block));
    }

    let padding = *decrypted_blocks.last().unwrap().last().unwrap();

    decrypted_blocks
        .into_iter()
        .flatten()
        .take(ciphertext.len() - padding as usize)
        .collect::<Vec<u8>>()
}

fn xor_bytes(x_bytes: &[u8], y_bytes: &[u8]) -> Vec<u8> {
    x_bytes
        .iter()
        .zip(y_bytes.iter())
        .map(|(x_byte, y_byte)| x_byte ^ y_byte)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    #[test]
    fn test_c10() {
        let message = b"This not to be read";
        let key = b"YELLOW SUBMARINE";
        let iv = hex!("00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00");

        let ciphertext = encrypt(message, key, &iv);
        let result = decrypt(&ciphertext, key, &iv);

        assert_eq!(message, &result[..]);
    }
}
