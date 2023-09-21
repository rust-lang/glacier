pub trait QueryDb {
    type Db;
}

pub struct QueryTable<Q, DB> {
    db: DB,
    storage: Q,
}

impl<Q> QueryTable<Q, <Q as QueryDb>::Db> where Q: for<'d> AsyncQueryFunction<'d> {}

pub trait AsyncQueryFunction<'d>: QueryDb<Db = <Self as AsyncQueryFunction<'d>>::SendDb> {
    type SendDb: 'd;
}

pub trait QueryStorageOpsAsync<Q>
where
    Q: for<'d> AsyncQueryFunction<'d>,
{
}

fn main() {}
