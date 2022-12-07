use hex;
use std::fs;
use std::path::Path;

pub fn read_bytes<P>(path: P) -> Vec<u8>
where
    P: AsRef<Path>,
{
    let base64_string = fs::read_to_string(path)
        .map(|res| res.replace('\n', ""))
        .expect("Error reading file!");

    base64::decode(base64_string).expect("Error decoding file!")
}

pub fn read_lines<P>(path: P) -> Vec<Vec<u8>>
where
    P: AsRef<Path>,
{
    let contents = fs::read_to_string(path).expect("Error reading file!");

    contents
        .lines()
        .map(|s| hex::decode(s).expect("Error decoding file!"))
        .collect()
}
