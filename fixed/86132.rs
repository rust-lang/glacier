#![crate_type = "staticlib"]
use std::mem;
trait Trait {}
const TRAIT_OBJ_UNALIGNED_VTABLE: &Trait =
    unsafe { mem::transmute((&92u8, &[0b_______001_11i128; 128])) };
