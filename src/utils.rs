use std::fs;

pub fn read_bytes(path: &str) -> Vec<u8> {
    let base64_string = fs::read_to_string(path)
        .map(|res| res.replace('\n', ""))
        .unwrap();

    base64::decode(base64_string).unwrap()
}
