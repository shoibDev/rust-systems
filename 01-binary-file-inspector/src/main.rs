use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path: &str = &args[1];
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("error: {e}");
            return;
        }
    };

    let mut buffer: [u8; 8] = [0; 8];
    let n = file.read(&mut buffer[..]).expect("Failed to read File");

    for i in 0_usize..n {
        println!("{}", buffer[i]);
    }
}

fn byte_to_bits(mut byte: u8) -> [u8; 8] {
    let mut byte_arr: [u8; 8] = [0; 8];

    for i in (0_usize..=7).rev() {
        byte_arr[i] = byte & 1;
        byte >>= 1;
    }

    byte_arr
}

fn byte_to_hex(byte: u8) -> String {
    let byte_arr = byte_to_bits(byte);
    let mut hex_str: String = String::new();

    let high = byte >> 4;
    let low = byte & 0x0F;

    match high {
        0..=9 => hex_str.push_str(&high.to_string()),
        10 => hex_str.push_str("A"),
        11 => hex_str.push_str("B"),
        12 => hex_str.push_str("C"),
        13 => hex_str.push_str("D"),
        14 => hex_str.push_str("E"),
        15 => hex_str.push_str("F"),
        _ => panic!("Invalid"),
    };

    match low {
        0..=9 => hex_str.push_str(&low.to_string()),
        10 => hex_str.push_str("A"),
        11 => hex_str.push_str("B"),
        12 => hex_str.push_str("C"),
        13 => hex_str.push_str("D"),
        14 => hex_str.push_str("E"),
        15 => hex_str.push_str("F"),
        _ => panic!("Invalid"),
    };

    hex_str
}
