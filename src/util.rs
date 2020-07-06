extern crate base64;
extern crate hex;

pub fn hex_string_to_base64(hex_string: String) -> String {
    let bytes = hex::decode(hex_string);
    base64::encode(bytes.unwrap())
}
