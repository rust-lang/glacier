#![feature(adt_const_params, generic_const_exprs)]

struct Changes<const CHANGES: &[&'static str]>
where
    [(); CHANGES.len()]:, {}

impl<const CHANGES: &[&str]> Changes<CHANGES> where [(); CHANGES.len()]: {}

fn main() {}
