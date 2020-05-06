fn test_ref(x: &u32) -> impl std::future::Future<Output = u32> + '_ {
    *x
}

fn main() {
    let _ = test_ref &u;
}
