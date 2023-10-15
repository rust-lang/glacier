#![no_std]
#![no_core]
#![no_main]
#![feature(no_core, lang_items)]

#[lang = "sized"]
trait Sized {}
#[lang = "sync"]
trait Sync {}
#[lang = "copy"]
trait Copy {}
#[lang = "freeze"]
trait Freeze {}

impl Sync for u32 {}
impl Sync for i32 {}

// impl Copy for u32 {} // if remove, it cause rustc crash

#[lang = "drop_in_place"]
#[allow(unconditional_recursion)]
pub unsafe fn drop_in_place<T: ?Sized>(to_drop: *mut T) {
    unsafe { drop_in_place(to_drop) }
}

#[link(name = "Kernel32")]
extern "system" {
    pub fn ExitProcess(uexitcode: u32) -> !;
}

fn exit_process(exit_code: u32) -> ! {
    // Copying is necessary to pass the value through the function argument, and if this is avoided, there will be no error either.
    unsafe {
        ExitProcess(exit_code);
    }
}

#[no_mangle]
pub extern "C" fn wWinMainCRTStartup() -> i32 {
    exit_process(0);
}

#[no_mangle]
pub static _fltused: i32 = 1;

fn main() {}
