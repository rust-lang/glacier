#![feature(fn_traits)]
#![feature(adt_const_params)]

#[derive(PartialEq, Eq)]
struct CompileTimeSettings{
    hooks: &'static[fn()],
}

struct Foo<const T: CompileTimeSettings>;

impl<const T: CompileTimeSettings> Foo<T> {
    fn callHooks(){
    }
}

fn main(){
    const settings: CompileTimeSettings = CompileTimeSettings{
        hooks: &[],
    };
    
    Foo::<settings>::callHooks();
}
