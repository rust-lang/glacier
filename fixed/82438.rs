fn main() {
    let inner;
    let outer = || {
        inner = || {};
        inner();
    };
    outer();
}
