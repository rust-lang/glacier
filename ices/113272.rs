trait Trait {
    type RefTarget;
}

impl Trait for () where Missing: Trait {}

struct Other {
    data: <() as Trait>::RefTarget,
}

fn main() {
    unsafe {
        std::mem::transmute::<Option<()>, Option<&Other>>(None);
    }
}
