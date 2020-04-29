#![feature(llvm_asm)]

extern "C" fn foo() { }

fn main() {
    unsafe {
        llvm_asm!("callq $0" :: "s"(foo) :: "volatile");
    }
}
