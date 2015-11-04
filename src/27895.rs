fn main() {
    let i = 5;
    let index = 6;

    match i {
        0...index => println!("winner"),
        _ => println!("hello"),
    }
}
