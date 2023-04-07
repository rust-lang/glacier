#![feature(dyn_star)]

const _: dyn* Send = &();
// static V: dyn* Send = Sync = &();

fn main() {}
