#![crate_type = "lib"]

use std::collections::HashMap;

struct Widget {}

pub trait WidgetExt {
    fn set_callback<F: FnMut(&mut Self) + 'static>(&mut self, cb: F)
    where
        Self: Sized;
}

static CALLBACKS: HashMap<*const dyn WidgetExt, dyn FnMut(&mut _) + 'static> = HashMap::new();
