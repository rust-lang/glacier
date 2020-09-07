fn main() {
    const CONST: &[Option<()>; 1] = &[Some(())];
    match &[Some(())] {
        &[None] => {}
        CONST => {}
        &[Some(())] => {}
    }
}
