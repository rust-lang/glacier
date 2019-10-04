fn main() {
    {
        extern "C" fn test(a: &mut i32) {
            println!("a: {:?}", a);
            *a = 1;
        }
    }
    {
        extern "C" fn test(b: &i32) {
            println!("b: {:?}", b);
        }
        test(&1);
    }
}
