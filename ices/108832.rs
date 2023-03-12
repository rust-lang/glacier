fn ice()
where
    for<'w> fn(&'w ()): FnMut(&'w ()),
{
}

fn main() {
    ice();
}
