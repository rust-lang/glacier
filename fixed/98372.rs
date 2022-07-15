#![feature(
    no_core, lang_items, intrinsics, unboxed_closures, type_ascription, extern_types,
    untagged_unions, decl_macro, rustc_attrs, transparent_unions, auto_traits,
    thread_local
)]
#![no_core]

#[lang = "sized"]
pub trait Sized {}

#[lang = "unsize"]
pub trait Unsize<T: ?Sized> {}

#[lang = "coerce_unsized"]
pub trait CoerceUnsized<T> {}

#[lang = "copy"]
pub trait Copy {}

#[lang = "sync"]
pub unsafe trait Sync {}

unsafe impl Sync for [u8; 16] {}

pub trait Allocator {
}

pub struct Global;

impl Allocator for Global {}

#[lang = "owned_box"]
pub struct Box<
    T: ?Sized,
    A: Allocator = Global,
>(*mut T, A);

pub fn main() {}
