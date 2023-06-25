#![feature(const_trait_impl)]

const fn test() -> impl ~const Fn() {
    const move || {
        let sl: &[u8] = b"foo";

        match sl {
            [first, _remainder @ ..] => {
                assert_eq!(first, &b'f');
            }
            [] => panic!(),
        }
    }
}

fn main() {}
