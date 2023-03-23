macro_rules! m {
    () => {
        macro_rules! foo {
            () => {}
        }
        use foo as bar;
    }
}

m!{}

use bar as baz;

baz!{}

fn main() {}
