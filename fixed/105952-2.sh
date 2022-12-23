#!/bin/bash

cat > foo.rs << EOF
#![feature(associated_const_equality)]
pub enum ParseMode {
    Raw,
}
pub trait Parse {
    const PARSE_MODE: ParseMode;
}
pub trait RenderRaw: Parse<PARSE_MODE = { ParseMode::Raw }> {}

EOF

rustdoc foo.rs
