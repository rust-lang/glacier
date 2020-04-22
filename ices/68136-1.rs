#![feature(asm)]

extern "C" fn foo() { }

fn main() {
    unsafe {
        asm!("callq $0" :: "s"(foo) :: "volatile");
    }
}
