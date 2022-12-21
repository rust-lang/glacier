#!/bin/bash

rustc "--edition=2018" "-Zdrop-tracking" "--crate-type=lib" - << EOF
pub struct File<B> {
    block: B,
}

pub async fn commit<B: Clone>(this: &mut File<B>) {
    async {}.await;
    async {}.await;
    async {}.await;
    async {}.await;

    let file = async { &this }.await;
    *async { &mut this.block }.await = file.block.clone();
}

EOF
