use anyhow::{bail, Result};
use glacier::Outcome;
use rayon::prelude::*;

fn main() -> Result<()> {
    let failed = glacier::test_all()?
        .map(|res| -> Result<bool> {
            let result = res?;
            println!("{}", result);

            Ok(result.outcome() != Outcome::ICEd)
        })
        .try_reduce(|| false, |a, b| Ok(a || b))?;

    if failed {
        bail!("some ICEs are now fixed!");
    } else {
        Ok(())
    }
}
