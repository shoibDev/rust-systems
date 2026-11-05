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

fn byte_conversion(mut x: u8) -> [u8; 8] {
    let mut byte_arr: [u8; 8] = [0; 8];

    for i in (0_usize..=7).rev() {
        byte_arr[i] = x % 2;
        x /= 2;
    }

    for element in byte_arr {
        print!("{element}");
    }
    byte_arr
}

fn hex_conversion(byte_arr: [u8; 8]) {
    let mut hex_str: String = String::new();

    for i in 0..=1 {
        let mut hex_val: u8 = 0;
        let offset: usize = 4 * i;

        for i in (0_usize..=3).rev() {
            if byte_arr[i + offset] == 1 {
                hex_val += 2_u8.pow((3 - i) as u32);
            }
        }

        match hex_val {
            0..=9 => hex_str.push_str(&hex_val.to_string()),
            10 => hex_str.push_str("A"),
            11 => hex_str.push_str("B"),
            12 => hex_str.push_str("C"),
            13 => hex_str.push_str("D"),
            14 => hex_str.push_str("E"),
            15 => hex_str.push_str("F"),
            _ => panic!("Invalid"),
        };
    }
    println!("{hex_str}");
}
