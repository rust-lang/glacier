trait A where
    for<'a> Self: 'a,
{
}
