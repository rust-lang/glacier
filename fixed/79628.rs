trait Output<'a> {
    type Type;
    fn convert() -> Self::Type;
}

impl<'a> Output<'a> for i32 {
    type Type = i32;
    
    fn convert() -> i32 {
        0
    }
}

struct Wrapper;

impl Wrapper {
    fn do_something_wrapper<O, F>(&mut self, use_output: F)
    where
        O: for<'a> Output<'a>,
        F: for<'a> FnOnce(<O as Output<'a>>::Type)
    {
        let output = <O as Output>::convert();
        use_output(output)
    }
}

fn main() {
    let mut wrapper = Wrapper;
    wrapper.do_something_wrapper::<i32, _>(|value| ());
}
