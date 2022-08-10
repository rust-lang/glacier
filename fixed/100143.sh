#!/bin/bash

cat > out.rs <<'EOF'

// struct Peekable<I: Iterator>
use std::iter::Peekable;

pub struct Span<F: Fn(&i32)> {
    inner: Peekable<ConditionalIterator<F>>,
}

struct ConditionalIterator<F> {
    f: F,
}

impl<F: Fn(&i32)> Iterator for ConditionalIterator<F> {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

EOF

rustdoc --edition=2021 out.rs

