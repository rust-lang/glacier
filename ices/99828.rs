fn get_iter(vec: &[i32]) -> impl Iterator<Item = {}> + '_ {
    vec.iter()
}

fn main() {
    let vec = Vec::new();
    let mut iter = get_iter(&vec);
    iter.next();
}
