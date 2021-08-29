use std::ops::Index;

pub trait Indexable {
    type Index;
}
struct Foo;
struct Bar;
impl Indexable for Foo { type Index = u8; }
impl Indexable for Bar { type Index = u16; }

pub trait Indexer<T: Indexable>: Index<T::Index, Output=T> {}

struct Store;

impl Index<u8> for Store {
    type Output = Foo;
    fn index(&self, _: u8) -> &Foo { panic!() }
}
impl Index<u16> for Store {
    type Output = Bar;
    fn index(&self, _: u16) -> &Bar { panic!() }
}
impl Indexer<Foo> for Store { }
impl Indexer<Bar> for Store { }

// implies StoreIndex: Index<u8, Output=Foo> + Index<u16, Output=Bar>
trait StoreIndex: Indexer<Foo> + Indexer<Bar> {}

impl StoreIndex for Store {}

struct Collection {
    stores: Vec<Store>,
}

trait StoreCollection {
    fn get_store(&self, _: usize) -> Option<&dyn StoreIndex>;
}

impl StoreCollection for Collection {
    //  Fails to compile:
    //    expected:
    //      Option<&dyn StoreIndex<Output = Bar, Output = Foo>
    //    found:
    //      Option<&Store>
    /*
    fn get_store(&self, i: usize) -> Option<&dyn StoreIndex> {
        self.stores.get(i)
    }
    */

    // ICE
    fn get_store(&self, i: usize) -> Option<&dyn StoreIndex> {
        if let Some(s) = self.stores.get(i) {
            Some(s as &dyn StoreIndex)
        } else {
            None
        }
    }

    // However, if the above is removed in favor of Indexing
    // type checking succeeds and the type of `&self.stores[i]`
    // is properly inferred
    /*
    fn get_store(&self, i: usize) -> Option<&dyn StoreIndex> {
        if i < self.stores.len() {
            Some(&self.stores[i])
        } else {
            None
        }
    }
    */
}

fn main() {}
