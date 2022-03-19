rustc  --crate-type lib --edition 2018 -Cdebuginfo=2 - 2>&1 << EOF


trait Trait {}

pub fn run(_: &dyn FnOnce(&()) -> Box<dyn Trait + '_>) {}


EOF
