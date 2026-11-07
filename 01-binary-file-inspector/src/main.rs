use std::env;
use std::fs::File;
use std::io::{ErrorKind, Read};

const BYTES_PER_ROW: usize = 8;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: binary-file-inspector <file-path>");
        std::process::exit(1);
    }

    let path: &str = &args[1];
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) if e.kind() == ErrorKind::NotFound => {
            eprintln!("File not found");
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("Failed to open file: {e}");
            std::process::exit(1);
        }
    };

    let mut buffer: [u8; BYTES_PER_ROW] = [0; BYTES_PER_ROW];
    let mut offset: usize = 0;

    println!("{:<8} | {:<24} | {:<72} | ASCII", "OFFSET", "HEX", "BINARY"); // Column Headers
    loop {
        let mut bytes_read: usize = 0;
        // Keep reading until the buffer is full or EOF is reached.
        while bytes_read < BYTES_PER_ROW {
            let n = match file.read(&mut buffer[bytes_read..]) {
                Ok(n) => n,
                Err(e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e) => {
                    eprintln!("Error reading file, {}", e);
                    std::process::exit(1);
                }
            };

            if n == 0 {
                break;
            }

            bytes_read += n;
        }

        if bytes_read == 0 {
            break;
        }

        let mut hex_str = String::new();
        let mut binary_str = String::new();
        let mut ascii_str = String::new();

        for byte in buffer.iter().take(bytes_read) {
            hex_str.push_str(&byte_to_hex(*byte));
            hex_str.push(' ');
        }

        for byte in buffer.iter().take(bytes_read) {
            let byte_arr = byte_to_bits(*byte);

            for bit in byte_arr {
                binary_str.push(if bit == 0 { '0' } else { '1' });
            }

            binary_str.push(' ');
        }

        for byte in buffer.iter().take(bytes_read) {
            ascii_str.push(byte_to_ascii(*byte));
        }
        println!("{offset:08X} | {hex_str:<24} | {binary_str:<72} | {ascii_str}");
        offset += bytes_read; // Update the offset
    }
    println!("Bytes inspected: {offset}");
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

// Converts a 4-bit value (0-15) into a hexadecimal character.
fn hex_digit_to_char(hex: u8) -> char {
    match hex {
        0u8..=9 => (b'0' + hex) as char,
        10..=15 => (b'A' + (hex - 10)) as char,
        _ => '.',
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_byte_to_hex() {
        assert_eq!(byte_to_hex(0), "00");
        assert_eq!(byte_to_hex(10), "0A");
        assert_eq!(byte_to_hex(15), "0F");
        assert_eq!(byte_to_hex(16), "10");
        assert_eq!(byte_to_hex(255), "FF");
    }
    #[test]
    fn test_byte_to_ascii() {
        assert_eq!(byte_to_ascii(65), 'A');
        assert_eq!(byte_to_ascii(32), ' ');
        assert_eq!(byte_to_ascii(10), '.');
        assert_eq!(byte_to_ascii(0), '.');
        assert_eq!(byte_to_ascii(255), '.');
    }

    #[test]
    fn test_byte_to_bits() {
        assert_eq!(byte_to_bits(0), [0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(byte_to_bits(8), [0, 0, 0, 0, 1, 0, 0, 0]);
        assert_eq!(byte_to_bits(255), [1, 1, 1, 1, 1, 1, 1, 1]);
    }
}
