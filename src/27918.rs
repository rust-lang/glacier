fn empty_and_catchall(raw: &[u8]) {
    let mut iter = raw.split(|&b| b == b'/');
    let first = iter.next().unwrap();
    let second = iter.next();

    const EMPTY: &'static [u8] = b"";

    match (first, second) {
        (EMPTY, Some(EMPTY))  => (),
        _ => (),
    }
}

fn main() { empty_and_catchall(b"foo") }
