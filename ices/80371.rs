pub struct Header<'a> {
    pub value: &'a [u8],
}

pub fn test() {
    let headers = [Header{value: &[]}; 128];
}

fn main() {}
