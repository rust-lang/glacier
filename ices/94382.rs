#![feature(platform_intrinsics)]
#![feature(staged_api)]
struct u16x2(u16, u16);
extern "platform-intrinsic" {
    #[rustc_const_stable(feature = "foo", since = "1.3.37")]
    fn simd_extract<T, U>(x: T, idx: u32) -> U;
}
fn main() {
    const U: u16x2 = u16x2(13, 14);
    const V: u16x2 = U;
    const Y0: i8 = unsafe { simd_extract(V, 0) };
}
