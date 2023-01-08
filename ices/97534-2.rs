macro_rules! foo {
    () => {};
}

macro_rules! m {
    () => {
        use foo as bar;
    };
}

m! {}

use bar as baz;

baz! {}

fn main() {}
