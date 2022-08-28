#![crate_type = "lib"]
pub fn get_u16_from_unaligned_manual(data: [u8; 1]) -> u8 {
    data[0] << 8
}

