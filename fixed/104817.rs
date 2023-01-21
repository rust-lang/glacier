#![feature(type_alias_impl_trait)]
#![feature(specialization)]
#![allow(incomplete_features, dead_code)]

trait OpaqueTrait {}
impl<T> OpaqueTrait for T {}
type OpaqueType = impl OpaqueTrait;
fn mk_opaque() -> OpaqueType {
    || 0
}
trait AnotherTrait {}
impl<T: Send> AnotherTrait for T {}
impl AnotherTrait for OpaqueType {}

fn main() {}
