#![feature(staged_api)]
struct u16x2(u16, u16);
extern "platform-intrinsic" {
    #[rustc_const_stable(feature = "foo", since = "1.3.37")]
    fn simd_insert(x: T, idx: u32, val: U);
}
fn main() {
    const U: u16x2 = u16x2(13, 14);
    const { simd_insert(U, 0x1319_8a2e, 42_u16) }
}
