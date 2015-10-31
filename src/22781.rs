use std::collections::HashMap;
use std::collections::hash_map::Entry::Vacant;

pub fn main() {
    type F = Box<Fn(&()) + 'static>;
    let mut map: HashMap<(), F> = HashMap::new();
    let x: &mut F = match map.entry(()) {
        Vacant(_) => unimplemented!(),
        _ => unimplemented!()
    };
}
