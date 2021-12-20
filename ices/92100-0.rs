fn main() {
    let v = vec![0123456789];
    let mid = v.len() / 2;
    let s = &v;
    match s {
        [a..mid, mid, mid..b] => {}
        [..] => {}
    }
}
