enum Foo {}
impl Drop for Foo {
    fn drop(&mut self) {}
}

fn main() {
    unsafe { std::ptr::read(&1u8 as *const u8 as *const Foo) };
}
