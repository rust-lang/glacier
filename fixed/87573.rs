#![crate_type = "lib"]
#![feature(no_core, lang_items)]
#![no_core]

static STATIC_BOOL: bool = true;

#[lang = "sized"]
trait Sized {}

#[lang = "copy"]
trait Copy {}

// Uncommenting avoids the second ICE
//impl Copy for bool {}

#[lang = "sync"]
trait Sync {}
impl Sync for bool {}

// Both errors occur here and are independent
#[lang = "drop_in_place"]
fn drop_fn() { // <- the drop fn expects a generic T, missing T causes index error
    while false {} // <- this expects the bool condition to be Copy, and it isn't
}

// required to make the compiler happy
#[lang = "start"]
fn start(){}
