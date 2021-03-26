#[lang = "fn"]
trait Fn {
    fn call(export_name);
}
fn call_through_fn_trait() {
    a()
}
