use std::{array, collections::HashMap};

fn main() {
    hex_conversion([0, 1, 0, 0, 1, 0, 0, 0]);
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
    let hex_map: HashMap<u8, &str> = HashMap::from([
        (10, "A"),
        (11, "B"),
        (12, "C"),
        (13, "D"),
        (14, "E"),
        (15, "F"),
    ]);

    let mut first_half: [u8; 4] = [0; 4];
    let mut second_half: [u8; 4] = [0; 4];

    first_half[0..=3].copy_from_slice(&byte_arr[0..=3]);
    second_half[0..=3].copy_from_slice(&byte_arr[4..=7]);

    let mut hex_val_1: u8 = 0;
    for i in (0_usize..=3).rev() {
        if first_half[i] == 1 {
            hex_val_1 += 2_u8.pow((3 - i) as u32);
        }
    }

    let mut hex_val_2: u8 = 0;
    for i in (0_usize..=3).rev() {
        if second_half[i] == 1 {
            hex_val_2 += 2_u8.pow((3 - i) as u32);
        }
    }

    match hex_val_1 {
        0..=9 => hex_str.push_str(&hex_val_1.to_string()),
        10..=15 => hex_str.push_str(hex_map.get(&hex_val_1).unwrap()),
        _ => print!("Fail"),
    };

    match hex_val_2 {
        0..=9 => hex_str.push_str(&hex_val_2.to_string()),
        10..=15 => hex_str.push_str(hex_map.get(&hex_val_2).unwrap()),
        _ => print!("Fail"),
    };

    println!("{hex_str}");
}
