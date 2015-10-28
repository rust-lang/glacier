#![feature(unboxed_closures, core)]

pub struct HttpConnector<T = NoSslVerify>(Option<T>);
pub struct NoSslVerify;

impl<'a> FnOnce<&'a mut u8> for NoSslVerify {
    type Output = ();
    #[inline(always)]
    extern "rust-call" fn call_once(self, _: (&mut u8)) {}
}

fn main() {}
