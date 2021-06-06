extern "C" fn _panic<'a, T: ?Sized>() -> Option<&'a T> {
    panic!()
}

fn main() {}
