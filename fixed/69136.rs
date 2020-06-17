#![feature(type_alias_impl_trait)]

trait SomeTrait where {}

trait WithAssoc<A> where {
	type AssocType;
}

type Return<A> // 'a is not in scope
    = impl WithAssoc<A, AssocType = impl SomeTrait + 'a>;

fn my_fun() -> Return<()> {}

fn main() {}
