#![crate_type = "lib"]

pub struct Memory<'rom> {
    rom: &'rom [u8],
    ram: [u8; Self::SIZE],
}

impl<'rom> Memory<'rom> {
    pub const SIZE: usize = 0x8000;
}
