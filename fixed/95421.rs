// Compile as binary
fn h5<T>(_: T)
where
    for<'x> fn(&'x u32): Fn(&'x u32),
{
}

fn main(){
    h5(|_|{})
}
