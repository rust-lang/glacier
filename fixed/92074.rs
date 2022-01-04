pub enum En {
    A(Vec<u8>)
}

fn f() -> Result<(), impl core::fmt::Debug> {
    let x: En = loop {};

    assert!(matches!(x, En::A(vec![])));
    Ok::<(), &'static str>(())
}
