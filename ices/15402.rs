#![feature(asm)]

#[repr(packed)]
struct Bitfield<T>(T);

fn test()->(Bitfield<u32>,Bitfield<u32>) {
    let mut out=(Bitfield(0),Bitfield(0));
    unsafe{asm!("" : "={eax}"(out.0), "={ebx}"(out.1))};
    out
}

fn main() {
    test();
}
