trait SomeTrait {}
impl SomeTrait for i32 {}
struct Component<T:?Sized>(T);
fn main() {
    let ref c = Component(42);
    let &Component(ref data) = c as &Component<SomeTrait>;
}
