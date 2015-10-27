enum Binding {}

fn main() {
    |binding: Binding| {
        if let Binding::None = binding {};
    };
}
