union U<'a> {
    x: &'a String,
}

fn f(u: U<'_>) -> String {
    unsafe { *u.x }
}

fn main() {}
