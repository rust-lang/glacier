#[allow(dead_code)]
#[repr(align(8))]
enum Aligned {
    Zero = 0,
    One = 1,
}

fn main() {
    let aligned = Aligned::Zero;
    println!("{}", aligned as u8);
}
