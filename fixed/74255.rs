#![feature(const_generics)]
#![allow(dead_code, incomplete_features)]

#[derive(PartialEq, Eq)]
enum IceEnum {
    Variant,
}

struct IceStruct;

impl IceStruct {
    fn ice_struct_fn<const I: IceEnum>() {}
}

fn ice_fn() {
    IceStruct::ice_struct_fn::<{ IceEnum::Variant }>();
}

fn main() {}
