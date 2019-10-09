use glacier::Outcome;
use rayon::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let failed = glacier::test_all()?
        .map(|res| -> Result<bool, String> {
            let result = res?;
            println!("{}", result);

            Ok(result.outcome() != Outcome::ICEd)
        })
        .try_reduce(|| false, |a, b| Ok(a || b))?;

    if failed {
        Err("some ICEs are now fixed!".into())
    } else {
        Ok(())
    }
}
