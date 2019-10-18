fn main() {
    test();
}

fn test() -> impl TraitA {
    test()
}

trait TraitA {}
