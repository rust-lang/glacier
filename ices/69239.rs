trait Trait<const S: &'static str> {}

struct Bug<T>
where
    T: Trait<{std::intrinsics::type_name::<T>()}>
{
    t: T
}

fn main() {}
