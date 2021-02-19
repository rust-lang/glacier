#![feature(let_chains)]

pub struct Describer {}

impl Describer {
    fn process_input(&mut self) {
        let index = "1";
        if let Ok(index) = index.parse::<usize>() && index < self.l2_items.len() {
            self.select(index);
        }
    }
}
