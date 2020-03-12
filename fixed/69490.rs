pub trait Trait<T> {
    const S: &'static str;
}

impl<T> Trait<()> for T
where 
    T: for<'a> Trait<std::marker::PhantomData<&'a ()>>,
{
    const S: &'static str = T::S;
}

fn main() {}
