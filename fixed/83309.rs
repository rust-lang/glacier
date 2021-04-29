fn main() {
    for v in Query.iter_mut() {
        *v -= 1;
    }
}

pub struct Query;
pub struct QueryIter<'a>(&'a i32);

impl Query {
    pub fn iter_mut<'a>(&'a mut self) -> QueryIter<'a> {
        todo!();
    }
}

impl<'a> Iterator for QueryIter<'a> {
    type Item = &'a i32;

    fn next(&mut self) -> Option<Self::Item> {
        todo!();
    }
}
