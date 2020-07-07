mod util;

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let line1 = stdin.lock().lines().next().unwrap().unwrap();
    let line2 = stdin.lock().lines().next().unwrap().unwrap();

    println!("{}", util::xor_hex_strings(line1, line2));
}
