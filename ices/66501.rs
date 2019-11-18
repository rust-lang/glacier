fn main() {
    const CONST: &[(); 1] = &[()];
    match &[()] {
        &[()] => {}
        CONST => {}
    }
}
