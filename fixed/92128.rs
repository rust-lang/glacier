#![feature(ptr_metadata)]

use std::alloc::Layout;
use std::ptr::Pointee;

trait RowOp {
    type Config;
}

struct Numbers;

impl RowOp for Numbers {
    type Config = ();
}

struct Func<T: RowOp> { _config: T::Config }

struct Inner<T: Pointee + ?Sized> { _meta: T::Metadata }

pub fn main() {
    // ICE caused by this line
    let _layout = Layout::new::<Inner<Func<Numbers>>>();
}
