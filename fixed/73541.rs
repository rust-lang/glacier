fn main() {
    'a: loop {
        async {
            loop {
                continue 'a;
            }
        };
    }
}
