fn main() {
    byte_conversion(7);
}

fn byte_conversion(mut x: u8) {
    let mut byte_arr: [u8; 8] = [0; 8];

    for i in (0_usize..=7).rev() {
        byte_arr[i] = x % 2;
        x /= 2;
    }

    for element in byte_arr {
        print!("{element}");
    }
}
