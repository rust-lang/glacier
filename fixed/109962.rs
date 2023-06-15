mod foo {
    pub mod bar {
        pub mod bar {
            pub mod bar {
              pub mod bar {}
            }
        }
    }
}
use foo::*;
use bar::bar::bar;
use bar::*;

fn main() {}