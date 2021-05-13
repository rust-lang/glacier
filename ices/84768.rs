#![feature(fn_traits)]

fn transform_mut<F>(f: F) where F: for<'b> FnOnce(&'b mut u8) {
    <F as FnOnce(&mut u8)>::call_once(f, 1)
}
