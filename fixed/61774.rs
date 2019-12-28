fn main() {
    let _: [_; unsafe { std::mem::transmute(|| {}) }];
}
