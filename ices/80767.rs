type G<'a> = [(O, &'a [u8]); 64];
enum O {
    O1,
}
fn f(_p: [G; 4]) {}

fn main() {
    let p: [G; 4];
    let _g: G = [(O::O1, &[]); 64];
    f(p);
}
