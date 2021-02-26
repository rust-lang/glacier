fn map<T>(_: fn() -> Option<&'static T>) -> Option<T> {
    None
}

fn value() -> Option<&'static _> {
    Option::<&'static u8>::None
}

const _: Option<_> = {

    let _: Option<_> = map(value);
};
