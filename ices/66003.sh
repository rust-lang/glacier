#!/bin/bash

git checkout https://github.com/olegnn/proc-macro-ICE.git && cd proc-macro-ICE

RUST_BACKTRACE=full cargo +nightly run
