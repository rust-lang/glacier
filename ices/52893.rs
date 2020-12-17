impl<P, T, Name, F, OLIST, FINAL> AddClass<Name, F> for Class<P, T>
where
    Self: At<Name>,
    <Class<P, T> as At<Name>>::AtRes: Push<F>,
    <<Class<P, T> as At<Name>>::AtRes as Push<F>>::PushRes:
        ToRef<Output = Class<P, OLIST>> + Push<OLIST>,
    <<<Class<P, T> as At<Name>>::AtRes as Push<F>>::PushRes as Push<OLIST>>::PushRes:
        Entry<Data = FINAL>,
{
    type Output = Class<P, FINAL>;

    fn init(self, func: F) -> Self::Output {
        let builder = self.at();
        let builder = builder.push(func);
        let output = builder.to_ref();
        builder.push(output)
    }
}

struct Class<P, T> {
    path: P,
    data: T,
}

trait At<Name> {
    type AtRes;

    fn at(self) -> Self::AtRes;
}

trait Push<T> {
    type PushRes;

    fn push(self, other: T) -> Self::PushRes;
}

trait AddClass<Name, F> {
    type Output;

    fn init(self, func: F) -> Self::Output;
}

trait Entry {
    type Data;
}

impl<P, T> Class<P, T> {
    fn with<Name, F>(self, constructor: F) -> <Self as AddClass<Name, F>>::Output
    where
        Self: AddClass<Name, F>,
    {
        loop {}
    }

    fn from<F>(self, constructor: F) -> <Self as AddClass<P, F>>::Output
    where
        Self: AddClass<P, F>,
    {
        loop {}
    }
}

trait ToRef {
    type Output;

    fn to_ref(&self) -> Self::Output;
}
