#![no_core]
fn main() {
    let one = || {};
    one()
}
#[lang = "sized"]
trait Sized {}
#[lang = "copy"]
trait Copy {}
