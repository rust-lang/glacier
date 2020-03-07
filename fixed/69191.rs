pub enum Void {}

pub trait Machine: Sized {
    type Seed: Sized;
    fn create(seed: Self::Seed) -> Response<Self, ()>;
}

pub struct Response<M, N>(std::marker::PhantomData<(M, N)>);
pub struct MyHandler<M>(std::marker::PhantomData<M>);
pub struct Loop<M>(std::marker::PhantomData<M>);

impl<M: Machine> Loop<M> {
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn add_machine_with<F>(&self, _fun: F)
    where
        F: FnOnce() -> Response<M, Void>,
    {
        unimplemented!()
    }

    pub fn run(self) {
        let handler = MyHandler::<M>::new();
        handler.timeout();
    }
}

impl<M: Machine> MyHandler<M> {
    pub fn new() -> Self {
        unimplemented!()
    }
    fn timeout(self) {
        let creator = None;
        let new = Some(creator.unwrap());
        M::create(new.unwrap());
    }
}

impl<M: Sized, N: Sized> Response<M, N> {
    pub fn map<T, U, S, R>(self, _: S, _: R) -> Response<T, U>
    where
        S: FnOnce(M) -> T,
    {
        unimplemented!()
    }
    pub fn wrap<T, S>(self, _: S) -> Response<T, N>
    where
        S: FnOnce(M) -> T,
    {
        unimplemented!()
    }
}

enum Tcp {}

enum Composed {
    Tcp(Tcp),
}
enum CSeed {
    Tcp(Void),
}

impl Machine for Composed {
    type Seed = CSeed;
    fn create(seed: CSeed) -> Response<Self, ()> {
        match seed {
            CSeed::Tcp(_x) => unimplemented!(),
        }
    }
}

impl Tcp {
    fn new() -> Response<Tcp, Void> {
        unimplemented!()
    }
}

impl Machine for Tcp {
    type Seed = Void;
    fn create(_seed: Void) -> Response<Self, ()> {
        unimplemented!()
    }
}

fn main() {
    let loop_creator = Loop::new();
    loop_creator.add_machine_with(|| Tcp::new().wrap(Composed::Tcp));
    loop_creator.run();
}
