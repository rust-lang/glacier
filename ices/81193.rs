#![feature(associated_type_bounds)]
trait A<'a, 'b> {}
trait B<'a, 'b, 'c> {}
fn err<'u, 'a, F>()
where
    for<'b> F: Iterator<Item: for<'c> B<'a, 'b, 'c> + for<'c> A<'a, 'c>>,
{
}
