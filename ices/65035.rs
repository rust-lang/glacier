fn _f<T>() {
    extern "C" {
        static _a: *const T;
    }
}

fn main() {}
