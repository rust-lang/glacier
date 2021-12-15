struct S<T>([T; 1]);

impl<K, V> S<(K, V)> {
    pub const fn foo(mut self, value: V) -> Self {
        let _arr = self.0;
        self.0[0].1 = value; //~ move
        self
    }
}

fn main() {}
