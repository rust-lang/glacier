#![warn(rust_2021_incompatible_closure_captures)]
impl Numberer {
    pub(crate) async fn new(
        interval: Duration,
