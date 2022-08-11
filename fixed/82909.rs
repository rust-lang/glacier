struct Sum(u32, u32);
impl PartialEq for Sum {
    fn eq(&self, other: &Self) -> bool {
        0 == 0
    }
}
#[derive(PartialEq)]
enum Eek {
    UnusedByTheConst(Sum),
}
const SUM_THREE: Eek = Eek::UnusedByTheConst(Sum(3, 0));
const EEK_ONE: &[Eek] = &[SUM_THREE];
fn main() {
    match &&&[][..] {
        &&EEK_ONE => {}
    }
}
