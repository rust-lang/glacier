rustc  --crate-type lib  --edition=2021 - 2>&1 << EOF

#![feature(generators)]
#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]

trait Service {
    type Future<'f>: std::future::Future + 'f
    where
        Self: 'f;

    fn spawn<'f>(&'f mut self) -> Self::Future<'f>;
}

struct Blah;

impl Service for Blah {
    type Future<'f> = impl std::future::Future<Output = &'static i32> + 'f;

    fn spawn<'f>(&'f mut self) -> Self::Future<'f> {
        async move {
            yield &1;
        }
    }
}


EOF
