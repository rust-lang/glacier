#[cfg(not(windows))]
use anyhow::{bail, Result};
#[cfg(not(windows))]
use glacier::{Outcome, TestResult};
#[cfg(not(windows))]
use rayon::prelude::*;

#[cfg(not(windows))]
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
        0 => eprintln!("\nFinished: No fixed ICEs"),
        len => bail!("{} ICEs are now fixed!", len),
    }

    Ok(())
}

#[cfg(windows)]
fn main() {
    println!(
        "We don't expect to run glacier on Windows. Please use WSL or *nix environment instead."
    );
}
