#![feature(associated_type_defaults)] 
trait Foo < T : Default + ToString >
{ 
  type Out : Default + ToString = ToString; 
}

impl Foo < u32 > for () {} 
impl Foo < u64 > for () {}

fn main ()
{ 
  assert_eq ! (< () as Foo < u32 >> :: Out :: default().to_string(), "false"); 
} 
