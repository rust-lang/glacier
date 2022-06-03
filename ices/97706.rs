pub fn compose(
    f1: impl FnOnce(f64) -> f64 + Clone,
    f2: impl FnOnce(f64) -> f64 + Clone,
) -> impl FnOnce(f64) -> f64 + Clone {
    move |x| f1(f2(x))
}

pub fn double(f: impl FnOnce(f64) -> f64 + Clone) -> impl FnOnce(f64) -> f64 + Clone {
    compose(f.clone(), f)
}


fn repeat_helper(f: impl FnOnce(f64) -> f64 + Clone, res: impl FnOnce(f64) -> f64 + Clone, times: usize) -> impl FnOnce(f64) -> f64 + Clone {
    if times == 1 {
        return res;
    }
    repeat_helper(f.clone(), compose(f, res), times - 1)
}

pub fn repeat(f: impl FnOnce(f64) -> f64 + Clone, times: usize) -> impl FnOnce(f64) -> f64 + Clone {
    repeat_helper(f.clone(), f, times)
}

pub fn main() {}
