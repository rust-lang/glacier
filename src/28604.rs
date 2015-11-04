#![feature(asm)]

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn get_timer() -> i32 {
    let tsc: i32;
    unsafe {
        asm!("nop": "=a"(tsc));
    }
    tsc
}

fn main() {
    println!("Val: {}", get_timer());
}
