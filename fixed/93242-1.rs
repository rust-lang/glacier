pub async fn something(path: &[usize]) -> usize {
    // Without this async block it doesn't ICE
    async {
        match path {
            [] => 0,
            // Adding the following match arm makes it not ICE, thankfully that makes this not a
            // blocker for me :)

            // [1] => 2,
            _ => 1,
        }
    }
    .await
}

fn main() {}
