#![feature(llvm_asm)]

fn test() -> () {
    unsafe {
        let test: i32;
        // using `str r0, [%0]` or any other instruction, it still crashes.
        llvm_asm!("mov %0, r0" : "=m"(test) ::);
    }   
}

fn main() {
    test();
}
