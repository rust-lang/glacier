#!/bin/bash

git clone https://github.com/thiolliere/debug-inconsistence.git && cd debug-inconsistence/runtime

cargo check --target=wasm32-unknown-unknown
