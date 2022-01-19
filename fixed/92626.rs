struct Test<T, const N: usize>([T; N]);

impl<T, const N: usize> std::ops::Deref for Test<T, N> {
    type Target = [T; N];
    
    fn deref(&self) -> &[T; N] {
        &self.0
    }
}

fn test() {
    let mut out = Test(todo!());
    let blah = out.len();
}

pub fn main() {}
