#!/bin/bash

rustc -Zunstable-options --pretty=expanded - << EOF
fn main(/*
    ---
*/) {}

EOF
