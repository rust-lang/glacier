#[repr(packed, align(0x1000))]
pub struct Foo {
    val: u16,
}

static BAR: Foo = Foo {
    val: 0,
};

fn main() {}
