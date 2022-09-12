use anyhow::{bail, Result};
use glacier::{Outcome, TestResult};
use rayon::prelude::*;

fn main() -> Result<()> {
    let filter = glacier::Filter::try_from_args(std::env::args())?;
    let failed = glacier::test_all_matching_filter(&filter)?
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
        1 => bail!("1 ICE ({paths:?}) is now fixed!"),
        len => bail!("{len} ICEs ({paths:?}) are now fixed!"),
    }

    Ok(())
}
