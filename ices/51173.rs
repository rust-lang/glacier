macro_rules! bug {
    ( $doc:expr ) => { #[doc = $doc] enum Bug {} }
}

bug! { (stringify!(bug)) }

fn main() {}
