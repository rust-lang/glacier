#![feature(label_break_value)]

fn main() {
    'a: {
        || break 'a
    }
}
