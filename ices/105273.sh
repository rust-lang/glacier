#!/bin/bash

rustc "-Zunpretty=ast-tree" - <<'EOF'

#![c = ({(while ""))();

EOF
