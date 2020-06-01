#!/bin/bash

rustc -C embed-bitcode=no -C debuginfo=2 --crate-type bin - << EOF

pub struct Thing;

impl Thing {
    pub async fn call(&self) -> Result<(), ()> {
        Ok(())
    }
}

async fn async_main() -> Result<(), ()> {
    // should be .call().await?
    Thing {}.call()?;
    Ok(())
}

fn main() {
    // Don't bother to block_on to avoid dependency on futures
    let _ = async_main();
}

EOF
