#![feature(closure_lifetime_binder)]

fn main() {
    for<const N: i32> || -> () {};    
}
