#![feature(slice_patterns)]

fn main() {
    match "foo".as_bytes() {
        b"food" => println!("nom nom"),
        [b'f', ..] => println!("f!"),
        _ => ()
    }
}
