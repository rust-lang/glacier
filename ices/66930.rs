#![crate_type = "lib"]

static UTF8_CHAR_WIDTH: [u8; 0] = [];

pub fn utf8_char_width(b: u8) -> usize {
    UTF8_CHAR_WIDTH[b as usize] as usize
}
