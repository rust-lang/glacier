#![feature(llvm_asm)]
unsafe fn test(x: *mut u32) {
    llvm_asm!("": "=m" (*x));
}

fn main() {
    let mut x = 5u32;
    let r = &mut x as *mut u32;
    
    unsafe {
        test(r);
    }
}
