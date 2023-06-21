use foo::*;

mod foo {
    pub mod bar {
        pub mod bar {
            pub mod bar {}
        }
    }
}

use bar::bar;
use bar::*;

fn main() {}
