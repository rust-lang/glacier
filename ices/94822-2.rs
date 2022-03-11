#![feature(lang_items)]

#[lang = "sized"]
trait A {}

fn a() {
    let _ = 0..80*25;
}
