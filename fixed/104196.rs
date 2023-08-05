use core::future::Future;

pub struct Struct<C, Fu, F>
where
    F: Fn(&C) -> Fu,
    Fu: Future,
{
    handler: F,
    context: C,
}

impl<C, Fu, R, F> Struct<C, Fu, F>
where
    F: Fn(&C) -> Fu,
    Fu: Future<Output = R>,
{
    pub const fn new(handler: F, context: C) -> Self {
        Self { handler, context }
    }
}

type TestC = &'static usize;
type TestFu = impl Future;
type TestF = impl Fn(&TestC) -> TestFu;
type TestStruct = Struct<TestC, TestFu, TestF>;

async fn test_handler(context: &TestC) -> () {
    ()
}

fn get_test_struct() -> &'static TestStruct {
    static test_actor: TestStruct = TestStruct::new(test_handler, &0);
    &test_actor
}

fn main() {}
