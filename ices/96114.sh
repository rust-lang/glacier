rustc -Zast-json - 2>&1 << EOF

#![feature(attr_literals)]
#[repr("C")] 
struct A {  }
fn main() {  }

EOF
