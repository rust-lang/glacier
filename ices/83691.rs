mod dep1 {
    extern "C" {
        pub static collision: Box<_>;
    }
}
fn main() {
    dep1::collision
}
