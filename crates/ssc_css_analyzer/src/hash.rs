#![allow(clippy::cast_possible_truncation, clippy::cast_lossless, clippy::cast_sign_loss)]

pub fn hash(input: &str) -> String {
    // Remove carriage return characters
    let input = input.replace('\r', "");

    // Initialize the hash value
    let mut hash: i32 = 5381;
    let mut i = input.len();

    // Compute the hash value
    while i > 0 {
        i -= 1;
        hash = ((hash << 5).wrapping_sub(hash)) ^ input.as_bytes()[i] as i32;
    }

    // Convert the hash to an unsigned 32-bit integer
    let hash_u32 = hash as u32;

    // Convert the hash to a base-36 string
    base36_encode(hash_u32)
}

fn base36_encode(mut num: u32) -> String {
    let mut result = String::new();
    while num > 0 {
        let remainder = num % 36;
        let digit = if remainder < 10 {
            (b'0' + remainder as u8) as char
        } else {
            (b'a' + (remainder - 10) as u8) as char
        };
        result.push(digit);
        num /= 36;
    }
    result.chars().rev().collect()
}
