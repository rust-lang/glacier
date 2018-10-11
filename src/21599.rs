#![no_std]
#![feature(box_syntax)]
#![feature(lang_items, ptr_internals)]

use core::panic::PanicInfo;
use core::ptr::Unique;

#[lang="owned_box"]
pub struct Box<T>(Unique<T>);

#[lang="start"]
fn main() {
    let mut test:[isize;1] = [0;1];
    let a = box 5;
    test[*a] = 0;
}

#[lang = "exchange_malloc"] extern fn exchange_malloc() {}
#[lang = "eh_personality"] extern fn eh_personality() {}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! { loop {} }
