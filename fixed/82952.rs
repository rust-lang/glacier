trait ExampleTrait<I> {}
type BlahResult<I, E = impl ExampleTrait<I>> = Result<I, E>;

fn example<'a, T>() -> BlahResult<()> {}
