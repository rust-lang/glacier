#![crate_type = "lib"]
#![feature(decl_macro)]

pub mod module {
    mod private {
        __helper__! { [$] recurse }
    
        macro __helper__ ([$dol:tt] $exported_name:ident) {
            macro_rules! $exported_name {() => ()}
            pub(crate) use $exported_name;
        }
            // pub(crate) use recurse;
    }
    pub(super) use private::recurse;
}

module::recurse!();
