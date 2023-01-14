#!/bin/bash

rustc -Zunpretty=ast-tree - << EOF
#![c={#![c[)x
EOF
