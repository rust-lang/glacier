macro_rules! jerry(
    ($name:expr) => {
        pub static NAME: *mut u8 = $name as *mut u8;
    }
);

fn main()  {
    jerry!(b"Kiwi\0");
}
