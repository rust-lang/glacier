pub trait WorldQuery {
    type Fetch: for<'world> Fetch<'world>;
}

fn main() {}

pub trait Fetch<'world> {
    type Item;
}

pub type QueryItem<'w, Q> = <<Q as WorldQuery>::Fetch as Fetch<'w>>::Item;

fn first_child() -> impl FnOnce(&u32) -> QueryItem<u32> {}