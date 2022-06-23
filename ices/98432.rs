struct Struct<T>(T);

impl<T> Struct<T> {
    const CONST: fn() = || {
        struct _Obligation where T:;
    };
}
