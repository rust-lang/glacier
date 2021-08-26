fn main() {
    enqueue::<(), _>(|_result| {});
}

trait Query<'a> {
    type Result: 'a;
    fn execute() -> Self::Result;
}

impl<'a> Query<'a> for () {
    type Result = ();
    fn execute() -> () {
        ()
    }
}

fn enqueue<Q: for<'q> Query<'q>, F: 'static + for<'r> FnOnce(<Q as Query<'r>>::Result)>(f: F) {
    let _: Box<dyn FnOnce()> = Box::new(move || f(Q::execute()));
}
