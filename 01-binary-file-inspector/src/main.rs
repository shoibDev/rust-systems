fn main() {
    byte_conversion(255);
}

fn byte_conversion(mut x: u8) {
    let mut byte_arr: [u8; 8] = [0; 8];
    let mut idx: usize = 7;

    while x > 0 {
        byte_arr[idx] = x % 2;
        x /= 2;

        if idx != 0 {
            idx -= 1;
        }
    }
    for element in byte_arr {
        print!("{element}");
    }
}
