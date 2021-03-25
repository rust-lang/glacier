#![warn(disjoint_capture_drop_reorder)]

fn main() {
    if let a = "" {
        drop(|_: ()| drop(a));
    }
}
