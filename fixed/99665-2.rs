pub trait MyComponent {
    type Properties;
}

impl<M> MyComponent for M
where
    M: 'static,
{
    type Properties = TableProps<M>;
}

pub struct TableProps<M>
where
    M: 'static,
{
    pub entries: M,
}

fn main() {
    |_: <&u32 as MyComponent>::Properties| {};
}
