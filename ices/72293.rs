#![feature(const_generics)]

struct Const<const P: &'static ()>;

fn main() {
    const A: &'static () = unsafe {
        std::mem::transmute(10 as *const ())
    };

    let _ = Const::<{A}>;
}
