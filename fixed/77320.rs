const FOO: isize = 10;
const ZST: &() = std::mem::transmute(FOO);
fn main() {
    match &() {
        ZST => 9,
    };
}
