unsafe fn setup_boostrap_code<A>(args: &A) {
    extern "C" {
        static xxx: *const A;
    }
    // Arguments
    let arg1_pointer_new: *const u64 = core::mem::transmute(&xxx);
}

fn main() {}
