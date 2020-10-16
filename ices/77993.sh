#!/bin/bash

rustc --edition 2018 - << EOF
async fn test() -> Box<dyn std::error::Error> {
    macro!()
}

fn main() {}

EOF
