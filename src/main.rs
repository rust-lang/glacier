use anyhow::{bail, Result};
use glacier::{Outcome, TestResult};
use rayon::prelude::*;

fn main() -> Result<()> {
    let failed = glacier::test_all()?
        .filter(|res| {
            if let Ok(test) = res {
                eprint!("{}", test.outcome_token());

                test.outcome() != Outcome::ICEd
            } else {
                true
            }
        })
        .collect::<Result<Vec<TestResult>, _>>()?;

    let mut paths = Vec::new();
    for result in &failed {
        eprintln!("\n{}", result);
        paths.push(result.path());
    }

    match failed.len() {
        0 => eprintln!("\nFinished: No fixed ICEs"),
        len => bail!("{len} ICEs ({:?}) are now fixed!", paths),
    }

    Ok(())
}
