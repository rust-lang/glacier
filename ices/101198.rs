#![no_std]
#![feature(lang_items, start)]
#![crate_type="lib"]

#[lang = "owned_box"]
struct Box<T>(*mut T);

#[lang = "exchange_malloc"]
unsafe fn alloc() -> *mut u8 {
    core::ptr::null_mut()
}

#[lang = "box_free"]
unsafe fn free<T: ?Sized>(ptr: *mut T) { }

impl<T> Box<T> {
    pub fn new(val: T) -> Box<T> {
        Box::new(val)
    }
}

#[start]
fn main(i: isize, args: *const *const u8) -> isize {
    let x = Box::new(3);
    0
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
