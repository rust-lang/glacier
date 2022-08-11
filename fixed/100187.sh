#!/bin/bash

rustc "-Zsave-analysis" - <<'EOF'

trait Pattern<'a> {}

async fn named_trait<'a, 'b>(foo: impl Pattern<'a>) -> impl Pattern<'b> {}

EOF

