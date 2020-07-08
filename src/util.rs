extern crate base64;
extern crate hex;
use std::collections::HashMap;
use std::iter;


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

pub fn xor_single_character(value: &Vec<u8>, key: u8) -> Vec<u8> {
    value.iter().zip(iter::repeat(key)).map(|(x, y)| x ^ y).collect()
}

pub fn english_score(string: &Vec<char>) -> f64 {
    let mut frequency: HashMap<char, f64> = HashMap::new();

    frequency.insert('a', 0.08167);
    frequency.insert('b', 0.01492);
    frequency.insert('c', 0.02782);
    frequency.insert('d', 0.04253);
    frequency.insert('e', 0.12702);
    frequency.insert('f', 0.02228);
    frequency.insert('g', 0.02015);
    frequency.insert('h', 0.06094);
    frequency.insert('i', 0.06094);
    frequency.insert('j', 0.00153);
    frequency.insert('k', 0.00772);
    frequency.insert('l', 0.04025);
    frequency.insert('m', 0.02406);
    frequency.insert('n', 0.06749);
    frequency.insert('o', 0.07507);
    frequency.insert('p', 0.01929);
    frequency.insert('q', 0.00095);
    frequency.insert('r', 0.05987);
    frequency.insert('s', 0.06327);
    frequency.insert('t', 0.09056);
    frequency.insert('u', 0.02758);
    frequency.insert('v', 0.00978);
    frequency.insert('w', 0.02360);
    frequency.insert('x', 0.00150);
    frequency.insert('y', 0.01974);
    frequency.insert('z', 0.00074);
    frequency.insert(' ', 0.13000);
    string.iter().fold(0.0, |acc, x| acc + frequency.get(x).unwrap_or(&0.0))
}
