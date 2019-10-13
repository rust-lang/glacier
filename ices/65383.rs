fn main() {
    let c = |p| {
        'main: for h in p..(p + 10) {
            let j = |k| {
                break 'main;
            };
            j(p);
        }
    };
    c(1);
}
