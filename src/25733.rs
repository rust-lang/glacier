fn main() {
    let x = [5; 100];
    foo(x);
}

fn foo<X: Copy>(x: X) { let _ = bar(x); }

fn bar<X:Clone>(x: X) { let _ = x.clone(); }
