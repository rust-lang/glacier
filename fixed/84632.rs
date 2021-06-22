macro_rules! n {() => (concat!("", n!()))}

fn main () {
    n!();
}
