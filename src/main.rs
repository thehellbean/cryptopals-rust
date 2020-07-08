mod util;

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();

    let bytes = hex::decode(line).unwrap();
    let english = decode_english(bytes).unwrap();
    let english_string: String = english.iter().collect();
    println!("{}", english_string);
}

fn decode_english(encoded: Vec<u8>) -> Option<Vec<char>> {
    let mut max_score = 0.0;
    let mut solution = None;
    
    for key in 0..=128 {
        let decoded= util::xor_single_character(&encoded, key).iter().map(|x| *x as char).collect();
        let score = util::english_score(&decoded);

        if score > max_score {
            max_score = score;
            solution = Some(decoded);
        }
    }         

    return solution;
}
