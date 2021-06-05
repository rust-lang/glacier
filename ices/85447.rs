#![crate_type="lib"]
use std::{collections::HashMap, hash::Hash};

struct C;

trait Ctx {
    type BindGroupId;
}

impl Ctx for C {
    type BindGroupId = u32;
}

pub struct BindGroup {
    _id: <C as Ctx>::BindGroupId,
}


pub fn insert(map: &mut HashMap<*const BindGroup, u32>, bind_group: *const BindGroup) {
    map.insert(bind_group, 1);
}
