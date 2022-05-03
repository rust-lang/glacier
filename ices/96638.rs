fn f(_: usize, _: &usize, _: usize) {}

fn arg<T>() -> T { todo!() }

fn main() {
    let x = arg(); // `x` must be inferred
    f(&x, ""); // The reference on `&x` is important to reproduce the ICE
}
