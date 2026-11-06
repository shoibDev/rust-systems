use std::env;
use std::fs::File;
use std::io::Read;

const BYTES_PER_ROW: usize = 8;

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

    let mut buffer: [u8; BYTES_PER_ROW] = [0; BYTES_PER_ROW];
    let mut offset: usize = 0;

    println!(
        "{:<8} | {:<24} | {:<72} | {}",
        "OFFSET", "HEX", "BINARY", "ASCII"
    ); // Column Headers
    loop {
        let bytes_read = file.read(&mut buffer).expect("Unexpected Error");

        if bytes_read == 0 {
            break;
        }

        let mut hex_str = String::new();
        let mut binary_str = String::new();
        let mut ascii_str = String::new();

        for i in 0..bytes_read {
            hex_str.push_str(&byte_to_hex(buffer[i]));
            hex_str.push(' ');
        }

        for i in 0..bytes_read {
            let byte_arr = byte_to_bits(buffer[i]);

            for bit in byte_arr {
                binary_str.push_str(&bit.to_string());
            }

            binary_str.push(' ');
        }

        for i in 0..bytes_read {
            ascii_str.push(byte_to_ascii(buffer[i]));
        }
        println!("{offset:08X} | {hex_str:<24} | {binary_str:<72} | {ascii_str}");
        offset += bytes_read; // Update the offset
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
