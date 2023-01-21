#!/bin/bash

rustc "-Zdrop-tracking" "--edition=2018" - << EOF
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

fn main() {
    let mut file = File {
        block: String::new(),
    };
    let _ = async move {
        commit(&mut file).await;
    };
}


EOF
