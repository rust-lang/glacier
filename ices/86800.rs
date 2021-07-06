#![feature(min_type_alias_impl_trait)]
use std::future::Future;

struct Connection {}

trait Transaction {}

struct TestTransaction<'conn> {
    conn: &'conn Connection,
}

impl<'conn> Transaction for TestTransaction<'conn> {}

struct Context {}

type TransactionResult<O> = Result<O, ()>;

type TransactionFuture<'__, O> = impl '__ + Future<Output = TransactionResult<O>>;

fn execute_transaction_fut<'f, F, O>(
    f: F,
) -> impl FnOnce(&mut dyn Transaction) -> TransactionFuture<O>
where
    F: FnOnce(&mut dyn Transaction) -> TransactionFuture<O> + 'f,
{
    f
}

impl Context {
    async fn do_transaction<O>(
        &self,
        f: impl FnOnce(&mut dyn Transaction) -> TransactionFuture<O>,
    ) -> TransactionResult<O> {
        let mut conn = Connection {};
        let mut transaction = TestTransaction { conn: &mut conn };
        f(&mut transaction).await
    }
}
