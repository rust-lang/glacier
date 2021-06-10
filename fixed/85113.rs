#![feature(min_type_alias_impl_trait)]
#![feature(impl_trait_in_bindings)]

type OpaqueOutputImpl<'a> = impl Output<'a> + 'a;

trait Output<'a> {}

impl<'a> Output<'a> for &'a str {}

fn cool_fn<'a>(arg: &'a str) -> OpaqueOutputImpl<'a> {
    let out: OpaqueOutputImpl<'a> = arg;
    arg // This was supposed to be `out`
}

fn main() {
    let s = String::from("wassup");
    cool_fn(&s);
}
