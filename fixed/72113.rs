#![feature(const_raw_ptr_deref)]

const OTHER_UNIT: &'static u8 = unsafe { &*(&&() as *const _ as *const u8) };

fn main() {}
