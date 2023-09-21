#![feature(async_fn_in_trait)]

fn main() {
    let _ = async {
        A.first().await.second().await;
    };
}

pub trait First {
    type Second: Second;
    async fn first(self) -> Self::Second;
}

struct A;

impl First for A {
    type Second = A;
    async fn first(self) -> Self::Second {
        A
    }
}

pub trait Second {
    async fn second(self);
}

impl<C> Second for C
where
    C: First,
{
    async fn second(self) {
        self.first().await.second().await;
    }
}
