#![crate_type = "lib"]
#![feature(lang_items)]
#![no_std]

struct NonNull<T: ?Sized>(*mut T);

struct Unique<T: ?Sized>(NonNull<T>);

#[lang = "owned_box"]
pub struct Box<T: ?Sized>(Unique<T>);

impl<T: ?Sized> Drop for Box<T> {
    fn drop(&mut self) {}
}

#[lang = "box_free"]
#[inline(always)]
unsafe fn box_free<T: ?Sized>(ptr: Unique<T>) {
    dealloc(ptr.0.0)
}

#[inline(never)]
fn dealloc<T: ?Sized>(_: *mut T) {}

pub struct Foo<T>(T);

pub fn foo(a: Option<Box<Foo<usize>>>) -> usize {
    let f = match a {
        None => Foo(0),
        Some(vec) => *vec,
    };
    f.0
}
