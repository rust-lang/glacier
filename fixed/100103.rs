#![feature(let_else)]
fn main() {
    let Some(x) = Some(()) else {
        match Err(()) {
            Err(()) => return (),
            Ok(val) => val,
        }
    };
}
