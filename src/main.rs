mod util;

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();

    println!("{}", util::hex_string_to_base64(line));
}
