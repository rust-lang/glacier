use x::y::z;
fn register_builtin_macros() {
    macro register_derive ($f:ident) {
        Box::z($f)
    }
    register_derive! { clone }
}
