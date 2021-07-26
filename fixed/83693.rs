#![feature(fn_traits)]
fn call() {
    <x as Fn(&usize)>::call
}
