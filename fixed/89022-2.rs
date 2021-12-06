#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![feature(const_fn_trait_bound)]

use core::marker::PhantomData;
use std::collections::HashMap;

// const-evaluable equality for string slices
pub const fn str_eq(lhs: &str, rhs: &str) -> bool {
    let lhs_bytes = lhs.as_bytes();
    let rhs_bytes = rhs.as_bytes();
    let mut i = 0;
    let bytes = if lhs_bytes.len() == rhs_bytes.len() {
        lhs_bytes.len()
    } else {
        return false;
    };

    while i < bytes {
        if lhs_bytes[i] != rhs_bytes[i] {
            return false;
        }
        i += 1;
    }
    return true;
}

pub trait ContainsKey<const K: &'static str> {}

// trait used to compare two types that have type-encoded lists of keys (in this cast static strings)
pub trait KeySchema {
    const KEYS: &'static [&'static str];
    const SIZE: usize;
}

pub struct KeyNil;
impl KeySchema for KeyNil {
    const KEYS: &'static [&'static str] = &[];
    const SIZE: usize = 0;
}

pub struct KeyCons<Tail, const KEY_ID: &'static str>
where
    Tail: KeySchema,
{
    _tail: PhantomData<Tail>,
}

pub const fn compute_successor_size<T: KeySchema>() -> usize {
    T::SIZE + 1
}

pub const fn construct_successor_array<Tail: KeySchema>(
    successor_key: &'static str,
) -> [&'static str; compute_successor_size::<Tail>()]
where
    [&'static str; compute_successor_size::<Tail>()]: Sized,
{
    let mut keys = [""; compute_successor_size::<Tail>()];
    let tail_keys = Tail::KEYS;
    let mut i = 0;
    let old_array_size: usize = compute_successor_size::<Tail>() - 1;
    while i < old_array_size {
        keys[i] = tail_keys[i];
        i += 1;
    }
    keys[old_array_size] = successor_key;
    keys
}

pub const fn is_equivalent_except<const K: &'static str>(
    with_k: &[&'static str],
    without_k: &[&'static str],
) -> bool {
    let mut i = 0;
    while i < with_k.len() {
        if str_eq(with_k[i], K) {
            i += 1;
            continue;
        }
        let mut j = 0;
        let mut match_found = false;
        while j < without_k.len() {
            if str_eq(with_k[i], without_k[j]) {
                match_found = true;
                break;
            }
            j += 1;
        }
        if !match_found {
            return false;
        }
        i += 1;
    }
    return true;
}

// Outputs a usize in order to make the array invalid by underflowing
// Alternatively this could use const_panic to produce good error messages
pub const fn check_valid_subset<S1: KeySchema, S2: KeySchema, const K: &'static str>() -> usize
where
    S1: ContainsKey<K>,
{
    let with_k: &[&'static str] = &S1::KEYS;
    let without_k: &[&'static str] = &S2::KEYS;

    if with_k.len() <= without_k.len() {
        // panic because S1 isn't bigger
        return (with_k.len() - 1) - without_k.len(); // panic using underflow
    }

    if !is_equivalent_except::<K>(with_k, without_k) {
        // panic because S2 doesn't have the rest of S1's elements
        return (without_k.len() - 1) - with_k.len(); // panic using underflow
    }

    return 1;
}

pub trait SubsetExcept<Parent: KeySchema, const K: &'static str>: KeySchema
where
    [(); Parent::SIZE - Self::SIZE]: Sized,
    Parent: ContainsKey<K>,
{
}

impl<Schema, PossibleParent, const K: &'static str> SubsetExcept<PossibleParent, K> for Schema
where
    Schema: KeySchema,
    PossibleParent: KeySchema,
    PossibleParent: ContainsKey<K>,
    [(); PossibleParent::SIZE - Schema::SIZE]: Sized,
    [(); check_valid_subset::<PossibleParent, Schema, K>()]: Sized,
{
}

impl<Tail, const KEY_ID: &'static str> KeySchema for KeyCons<Tail, KEY_ID>
where
    Tail: KeySchema,
    [(); compute_successor_size::<Tail>()]: Sized,
{
    const KEYS: &'static [&'static str] = &construct_successor_array::<Tail>(KEY_ID);
    const SIZE: usize = compute_successor_size::<Tail>();
}

// thanks to matt1992#5582 on the Rust Programming Language Community Discord for offering this strategy
// a const expression calls a function, which provides a "proof" that a given type should always use a given implementation
pub trait ContainsKeyHelper<const IS_EQUAL: bool, const K: &'static str> {}

pub const fn contains_key_helper_helper<const KEY_ID: &'static str, const K: &'static str>() -> bool
{
    str_eq(KEY_ID, K)
}
impl<Tail, const KEY_ID: &'static str, const K: &'static str> ContainsKey<K>
    for KeyCons<Tail, KEY_ID>
where
    Tail: KeySchema,
    Self: ContainsKeyHelper<{ contains_key_helper_helper::<KEY_ID, K>() }, K>,
{
}

impl<Tail, const KEY_ID: &'static str, const K: &'static str> ContainsKeyHelper<false, K>
    for KeyCons<Tail, KEY_ID>
where
    Tail: KeySchema + ContainsKey<K>,
{
}

impl<Tail, const KEY_ID: &'static str, const K: &'static str> ContainsKeyHelper<true, K>
    for KeyCons<Tail, KEY_ID>
where
    Tail: KeySchema,
{
}

pub struct RestrictedStringMap<S: KeySchema> {
    inner: HashMap<&'static str, Option<String>>,
    // schemas should be 0-sized, but I use a phantom data here just to emphasize that there's no data dependency
    _schema: PhantomData<S>,
}
impl<S: KeySchema, const K: &'static str> ContainsKey<K> for RestrictedStringMap<S> where
    S: ContainsKey<K>
{
}
impl<S: KeySchema> RestrictedStringMap<S>
where
    [(); compute_successor_size::<S>()]: Sized,
{
    pub fn empty_schema() -> RestrictedStringMap<KeyNil> {
        RestrictedStringMap::<_> {
            inner: HashMap::new(),
            // schemas should be 0-sized, but I use a phantom data here just to emphasize that there's no data dependency
            _schema: PhantomData::<_>,
        }
    }

    /// Adds a possible &'static str to the HashMap.
    /// This requires consuming the map since our type must change to reflect the new schema.
    pub fn add_key<const K: &'static str>(self) -> RestrictedStringMap<KeyCons<S, K>>
    where
        // Proof asserting that one size larger is still a valid schema
        // this should only be untrue if the number of keys exceeds usize::MAX
        [(); compute_successor_size::<S>()]: Sized,
    {
        let Self { mut inner, .. } = self;
        inner.insert(K, None);
        RestrictedStringMap::<_> {
            inner: inner,
            _schema: PhantomData::<_>,
        }
    }

    // I don't know of a way to remove the &'static str other than having the user provide their own new schema.
    // This is because I can't use a dependently typed function to construct a return type.
    // That's the only way I can think of to compute what the return type of such a function would look like without user input.
    pub fn remove_key<NewSchema: KeySchema, const K: &'static str>(
        self,
    ) -> RestrictedStringMap<NewSchema>
    where
        Self: ContainsKey<K>,
        S: ContainsKey<K>,
        // the schema that the user provides must be a valid subset of the old schema
        NewSchema: SubsetExcept<S, K>,
        [(); S::SIZE - NewSchema::SIZE]: Sized,
    {
        let Self { mut inner, .. } = self;
        inner.remove(&K);
        RestrictedStringMap::<_> {
            inner: inner,
            _schema: PhantomData::<_>,
        }
    }
}

fn foo() {
    let map: RestrictedStringMap<KeyNil> = RestrictedStringMap::<KeyNil>::empty_schema();
    let mut map: RestrictedStringMap<KeyCons<KeyCons<KeyNil, "k1">, "k2">> =
        map.add_key::<"k1">().add_key::<"k2">();
    let map: RestrictedStringMap<KeyCons<KeyNil, "k1">> = map.remove_key::<_, "k2">();
}

fn main() {}
