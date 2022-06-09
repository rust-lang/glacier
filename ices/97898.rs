fn main() {
    let x = "foo";
    
    match x {
        x @ ("foo" | "bar") |
        x @ ("red" | "blue") => {
        }
        _ => (),
    }
}
