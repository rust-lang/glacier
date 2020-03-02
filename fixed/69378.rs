struct Foo { 0: u8 }

fn test(f: Foo) {
    Foo{foo: 4, ..f};
}

fn main() {}
