extern crate proc_macro;

trait Project {
    type Assoc;
}

#[proc_macro]
fn uwu() -> <() as Project>::Assoc {}
