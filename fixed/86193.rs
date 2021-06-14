#![crate_type = "lib"]
#[repr(transparent)]
struct W<T>(T);

// The drop fn is checked before size/align are, so get ourselves a "sufficiently valid" drop fn
fn drop_me(_: *mut usize) {}

const INVALID_VTABLE_SIZE: W<&dyn Send> =
    unsafe { std::mem::transmute((&92u8, &(drop_me as fn(*mut usize), usize::MAX, 1usize))) };
