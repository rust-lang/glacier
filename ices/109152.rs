#![crate_type = "lib"]
fn _y() {
    vec![42].iter().map(drop);
}

fn main() {}
