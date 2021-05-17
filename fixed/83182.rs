use std::mem;
struct MyStr(str);
const MYSTR_NO_INIT: &MyStr = unsafe { mem::transmute::<&[_], _>(&[&()]) };

fn main() {}
