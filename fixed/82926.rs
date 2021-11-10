#![feature(staged_api)]
struct f32x3(f32, f32, f32);
extern "platform-intrinsic" {
    #[rustc_const_stable(feature = "foo", since = "1.3.37")]
    fn simd_insert<T>(x: T, idx: u32, val: U) -> T;
}
fn main() {
    const U: f32x3 = f32x3(13., 14., 15.);
    const { simd_insert(U, 1_u32, 42_f32) }
}
