unsafe fn f<T: ?Sized>(ptr: *mut T, new: *mut u8) -> *mut T {
    use std::mem;

    let mut parts: [*mut u8; mem::size_of::<*mut T>() / mem::size_of::<*mut u8>()] =
        mem::transmute(ptr);
    parts[0] = new;
    mem::transmute(parts)
}

fn main() {}
