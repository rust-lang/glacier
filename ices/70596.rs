#![feature(llvm_asm)]
extern "C" fn foo() {}

fn main() {
    unsafe {
        llvm_asm!("mov x0, $0"
            :: "r"(foo)
            :: "volatile");
    }
}
