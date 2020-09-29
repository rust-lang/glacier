struct Matrix<T>;

trait Allocator<T> {
    type Buffer;
}

struct ArrayStorage<T> {}

impl<T> Allocator<T> for DefaultAllocator {
    type Buffer = ArrayStorage<T>;
}

struct DefaultAllocator;

type A = impl Fn(Matrix<<DefaultAllocator as Allocator<()>>::Buffer>);

fn foo() -> A {
    |_| ()
}

fn main() {}
