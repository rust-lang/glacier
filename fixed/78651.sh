#!/bin/bash

rustc - <<EOF
use std::result;
impl result {
    fn into_future() -> Err {}
}
EOF
