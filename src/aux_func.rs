pub fn letter_to_int(ch: char) -> i32 {
    ch as i32 - 64
}

pub fn int_to_letter(num: i32) -> char {
    ((num + 64) as u8) as char
}