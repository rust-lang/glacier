struct Bug {
    incorrect_field: 0,
}

struct Empty {}

fn main() {
    let Bug {
        any_field: Empty {},
    } = Bug {};
}
