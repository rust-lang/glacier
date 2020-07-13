#![feature(const_generics, const_transmute)]

struct Const<const P: &'static ()>;

fn main() {
    const A: &'static () = unsafe {
        std::mem::transmute(10 as *const ())
    };

    let _ = Const::<{A}>;
}
