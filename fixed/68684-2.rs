enum _Enum {
    A(),
}

type _E = _Enum;

const fn _a() -> _Enum {
    _E::A() // Changing to `_Enum::A()` makes it compile.
}

const _A: _Enum = _a();

fn main() {}
