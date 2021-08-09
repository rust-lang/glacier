#[repr(transparent)]
struct TransparentCustomZst(());
extern "C" {
    fn good17(p: TransparentCustomZst);
}

fn main() {}
