#!/bin/bash

rustc -Zunsound-mir-opts - << EOF
// run-pass

#![allow(dead_code)]

#[derive(Debug)]
enum MyEnum {
    Variant2,
    Variant3,
}

fn f(arg3: bool) -> MyStruct {
	match if arg3 { Some(MyEnum::Variant3) } else { None } {
		Some(t) => {
			let ah = t;
			return MyStruct(Some(ah));
		}
		_ => MyStruct(None)
	}
}

#[derive(Debug)]
struct MyStruct(Option<MyEnum>);

fn main() {
    f(true);
}
EOF
