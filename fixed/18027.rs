#![feature(slice_patterns)]
fn main() {
    match "".as_bytes() {
        b"" => (),
        [] => (),
    }
}
