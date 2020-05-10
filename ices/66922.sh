#!/bin/bash

git clone https://github.com/95th/proc-macro-workshop.git && cd proc-macro-workshop

git checkout ICE

cargo +nightly run
