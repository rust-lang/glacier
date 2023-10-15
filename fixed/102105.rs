#![crate_type = "lib"]
#![feature(rustc_attrs)]

#[rustc_allocator]
pub fn allocator() -> *const i8 {
  std::ptr::null()
}
