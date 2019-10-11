mod fix;
mod github;

use fix::fix;
use glacier::rayon::prelude::*;
use glacier::{test_all, Outcome};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let config = github::Config::from_env()?;

    test_all()?
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .filter(|result| result.outcome() != Outcome::ICEd)
        .try_for_each(|result| fix(&result, &config))
}
