fn main() {
    (0..)
        .map(
            #[target_feature(enable = "")]
            |_| (),
        )
        .next();
}
