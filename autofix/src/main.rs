mod fix;
mod github;

use anyhow::{Context, Result};
use fix::fix;
use glacier::rayon::prelude::*;
use glacier::{test_all, Outcome};

fn main() -> Result<()> {
    let config = github::Config::from_env()?;

    test_all()?
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .filter(|result| result.outcome() != Outcome::ICEd)
        .try_for_each(|result| fix(&result, &config).context(result))
}
