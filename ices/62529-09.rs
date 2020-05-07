use std::marker::PhantomData;

// Borrowing encoding of paramaterized types from
// https://github.com/rust-lang/rfcs/blob/master/text/0195-associated-items.md#encoding-higher-kinded-types

trait TypeWithLifetime<'a> {
    type Type: Copy;
}

// type At<'a,T> where T: TypeWithLifetime<'a> = T::Type;

struct Str;

impl<'a> TypeWithLifetime<'a> for Str {
    type Type = &'a str;
}
    
trait Consumer<T> where T: for<'a> TypeWithLifetime<'a> {
    fn accept(&mut self, arg: <T as TypeWithLifetime>::Type);
}

impl Consumer<Str> for String {
    fn accept(&mut self, arg: &str) { self.push_str(arg) }
}

struct FilterConsumer<F,T,C> {
    function: F,
    consumer: C,
    phantom: PhantomData<T>,
}

impl<F,T,C> Consumer<T> for FilterConsumer<F,T,C> where F: Fn(<T as TypeWithLifetime>::Type) -> bool, T: for<'a> TypeWithLifetime<'a>, C: Consumer<T> {
    fn accept(&mut self, arg: <T as TypeWithLifetime>::Type) {
        if (self.function)(arg) { self.consumer.accept(arg) }
    }
}

fn main() {
    let mut consumer = FilterConsumer{
        function: |x:<Str as TypeWithLifetime>::Type| x.chars().all(char::is_alphabetic),
        consumer: String::new(),
        phantom: PhantomData,
    };
    consumer.accept("hi");
}
