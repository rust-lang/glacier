#![feature(slice_patterns)]
fn main() {
    let buf = [0u8; 4];
    match &buf {
        &[0, 1, 0, 0] => (),
        b"true" => (),
        _ => ()
    }   
}
