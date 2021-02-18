use std::sync::Mutex;

struct MarketMultiplier {}

impl MarketMultiplier {
    fn buy(&mut self) -> &mut usize {
        todo!()
    }
}

async fn buy_lock(generator: &Mutex<MarketMultiplier>) -> LockedMarket<'_> {
    LockedMarket(generator.lock().unwrap().buy())
}

struct LockedMarket<T>(T);

fn main() {}
