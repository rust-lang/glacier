rustc --edition 2021 - 2>&1 << EOF

#![feature(type_alias_impl_trait)]

use std::future::Future;

fn main() {
    let _ = move || async move {
        let value = 0u8;
        blah(&value).await;
    };
}

type BlahFut<'a> = impl Future<Output = ()> + Send + 'a;
fn blah<'a>(_value: &'a u8) -> BlahFut<'a> {
    async {}
}

EOF
