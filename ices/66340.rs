fn main() {
    println!("main.");
    foo(); // remove this line for compiler output A
}

#[deprecated(note=test)]
fn foo(){
    println!("Hello, world");
}
