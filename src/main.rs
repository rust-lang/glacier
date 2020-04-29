use anyhow::{bail, Result};
use glacier::{TestResult, Outcome};
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

    for result in &failed {
        eprintln!("\n{}", result);
    }

    match failed.len() {
        0 => eprintln!("Finished: No fixed ICEs"),
        len => bail!("{} ICEs are now fixed!", len)
    }

    Ok(())
}
