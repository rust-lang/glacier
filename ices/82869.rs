#![feature(asm)]

fn aarch64(a: f64, b: f64) {
    asm!("add {:d}, {:d}, d0", out(vreg) c, in(vreg) a, in("d1") {
        || {};
        b
    });
}
