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
    let mut hex_str = String::new();

    let high = byte >> 4;
    let low = byte & 0x0F;

    hex_str.push(hex_digit_to_char(high));
    hex_str.push(hex_digit_to_char(low));

    hex_str
}

fn byte_to_ascii(byte: u8) -> char {
    match byte {
        0x20..=0x7E => byte as char,
        _ => '.',
    }
}

// Takes in half of the hex only...
fn hex_digit_to_char(hex: u8) -> char {
    match hex {
        0u8..=9 => (b'0' + hex) as char,
        10..=15 => (b'A' + (hex - 10)) as char,
        _ => '.',
    }
}
