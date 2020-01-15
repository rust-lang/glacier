#![feature(unsized_locals)]

fn main() {
    let f: fn([u8]) = |_| {};
    
    let slice: Box<[u8]> = Box::new([1; 8]);
    
    f(*slice);
}
