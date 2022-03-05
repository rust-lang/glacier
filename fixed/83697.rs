#![feature(platform_intrinsics)]
#![feature(staged_api)]
struct i8x1(i8);
extern "platform-intrinsic" {
    #[rustc_const_stable(feature = "foo", since = "1.3.37")]
    fn simd_extract<T, U>(x: T, idx: u32) -> U;
}
fn main() {
    const U: i8x1 = i8x1(13);
    const V: i8x1 = U;
    const Y0: i8 = unsafe { simd_extract(V, 8) };
}
