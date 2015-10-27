fn main() {
    let x = T {f: Box::new([None])};
}

struct T {
    f: Box<[Option<T>; 1]>
}
