impl<P, T, Name, F, OLIST: HList, X, Y, FINAL> AddClass<Name, F> for Class<P, T>
where
    Self: At<Name>,
    P: Default,
    X: Entry,
    T: Push<(P, F)>,
    <X as Entry>::Data: Fn(Y) -> Class<P, OLIST>,
    <Class<P, T> as At<Name>>::AtRes: Push<(P, F)>,
    <<Class<P, T> as At<Name>>::AtRes as Push<(P, F)>>::PushRes: for<'this> ToRef<'this, Output=HCons<X, Y>> + Push<OLIST>,
    <<<Class<P, T> as At<Name>>::AtRes as Push<(P, F)>>::PushRes as Push<OLIST>>::PushRes: Entry<Data=FINAL>
{
    type Output = Class<P, FINAL>;

    fn init(self, func: F) -> Self::Output {
        let builder = self.at();
        let builder = builder.push((P::default(), func));
        let output = {
            let refs = builder.to_ref();
            let func = refs.head.borrow_data();
            func(refs.tail)
        };
        let final_data = builder.push(output);
        Class {
            path: P::default(),
            data: final_data.get_data()
        }
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

trait AddClass<Name, F>: At<Name> {
    type Output;

    fn init(self, func: F) -> Self::Output;
}

trait Entry {
    type Data;

    fn get_data(self) -> Self::Data;
    fn borrow_data(&self) -> &Self::Data;
}

impl<P: HList, T> Class<P, T> {
    fn with<Name, F>(self, constructor: F) -> <Self as AddClass<Name, F>>::Output
    where
        Self: AddClass<Name, F>,
    {
        self.init(constructor)
    }

    fn from<F>(self, constructor: F) -> <Self as AddClass<P, F>>::Output
    where
        Self: AddClass<P, F>,
    {
        self.init(constructor)
    }
}

fn main() {
    println!("Hello, world!");
}

/// Below structs are to bypass the original frunk requirements
pub trait HList {}
pub struct HCons<H, T> {
    pub head: H,
    pub tail: T,
}

pub trait ToRef<'a> {
    type Output;

    fn to_ref(&'a self) -> Self::Output;
}
