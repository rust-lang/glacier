fn arr_by_ref(mut x: [(); 3]) {
    let f = || {
        let [ref y, ref mut z @ ..] = x;
    };
    f();
}

fn main() {}
