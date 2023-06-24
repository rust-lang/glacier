pub trait MyComponent {
    type Properties;
}

impl<M> MyComponent for M
where
    M: 'static,
{
    type Properties = ();
}

fn main() {
    |_: <&u8 as MyComponent>::Properties| {};
}
