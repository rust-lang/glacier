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
