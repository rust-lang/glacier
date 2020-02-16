enum Enum {
    A(),
}

impl Enum {
    const fn a() -> Self {
        return Self::A();
    }
}

pub fn main() {
    const A: Enum = Enum::a();
}
