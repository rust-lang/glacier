macro_rules! width(
    ($this:expr) => {
        $this.width.unwrap()
    }
);

struct HasInfo {
    width: Option<usize>
}

impl HasInfo {
    fn get_size(&mut self, n: usize) -> usize {
        n
    }

    fn get_other(&mut self) -> usize {
        self.get_size(width!(self))
    }
}

fn main() {
    println!("hello?");
}

