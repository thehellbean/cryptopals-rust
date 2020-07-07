extern crate base64;
extern crate hex;

pub fn hex_string_to_base64(hex_string: String) -> String {
    let bytes = hex::decode(hex_string);
    base64::encode(bytes.unwrap())
}

pub fn xor_hex_strings(string1: String, string2: String) -> String {
    let bytes1 = hex::decode(string1).unwrap();
    let bytes2 = hex::decode(string2).unwrap();

    let xor_bytes: Vec<u8> = bytes1.iter().zip(bytes2.iter()).map(|(x, y)| x ^ y).collect();
    hex::encode(xor_bytes)
}
