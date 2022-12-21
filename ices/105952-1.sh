#!/bin/bash

cat > foo.rs << EOF
#![feature(associated_const_equality)]
pub enum ParseMode {
    Raw,
}
pub trait Parse {
    const PARSE_MODE: ParseMode;
}
pub trait RenderRaw {}
impl<T: Parse<PARSE_MODE = { ParseMode::Raw }>> RenderRaw  for T {}

EOF

rustdoc foo.rs
