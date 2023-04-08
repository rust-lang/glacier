use clippy::time::Instant;

fn main() {
    let prev_instant = Instant::now();

    let duration = prev_instant.elapsed();
}