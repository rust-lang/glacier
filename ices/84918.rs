#[lang = "shl_assign"]
trait ShlAssign {
    fn shl_assign(impl Clone);
}
macro_rules! shl_assign_impl {
    ( $ t : ty , $ f : ty ) => {
        impl t {
            fn shl_assign() {
                self <<= other
            }
        }
    };
}
macro_rules ! shl_assign_impl_all {
( $ ( $ t : ty ) * ) => ( $ (
shl_assign_impl !
                  { $ t , u16 }
)   )
}
shl_assign_impl_all! { u8                                                  }
