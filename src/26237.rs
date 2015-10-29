static mut TEST: [isize; 1] = [1];
static mut TEST2: &'static mut [isize] = unsafe { &mut TEST };
fn main() {
    println!("{}", unsafe { TEST2[0] });
}
