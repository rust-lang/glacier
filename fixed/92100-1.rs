fn main() {
    match &vec![0] {
        [a..1, a, a..1] | _ => {}
    }
}
