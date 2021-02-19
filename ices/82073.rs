#![feature(let_chains)]
#[async_std::main]
async fn main() -> Result<()> {
    if let Some(_) = 2 && let Some(sixteen) = 16 {
        isize::from_str_radix("A", sixteen)?;
    }
    Ok(())
}
