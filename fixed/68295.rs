pub struct Matrix<R, C, S> {
    _phantoms: std::marker::PhantomData<(R, C, S)>,
}

impl<R, C, S> Matrix<R, C, S> {
    pub fn into_owned(self) -> Matrix<R, C, Owned<R, C, ()>>
    where
        (): Allocator<R, C>,
    {
        unimplemented!()
    }
}

impl<D, S> Matrix<D, D, S> {
    pub fn hermitian_part(&self) -> Matrix<D, D, Owned<D, D, ()>>
    where
        (): Allocator<D, D>,
    {
        unimplemented!()
    }
}

pub trait Allocator<R, C> {
    type Buffer;
}

pub trait Trait<R, C, A> {
    type Power;
}

impl<R, C, A: Allocator<R, C>> Trait<R, C, A> for () {
    type Power = A::Buffer;
}

pub type Owned<R, C, G> = <G as Trait<R, C, ()>>::Power;

pub fn crash<R, C>(input: Matrix<R, C, ()>) -> Matrix<R, C, u32>
where
    (): Allocator<R, C>,
{
    input.into_owned()
}

fn main() {}
