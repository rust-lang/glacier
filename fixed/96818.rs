macro_rules! values {
    ($($token:ident($value:literal) $(as $inner:ty)? => $attr:meta,)*) => {
        #[derive(Debug)]
        pub enum TokenKind {
            $(
                #[$attr]
                $token $($inner)? = $value,
            )*
        }
    };
}

values!(STRING(1) as (String) => cfg(test),);

pub fn main() {}
