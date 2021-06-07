use crate::{Outcome, ICE};
use anyhow::Result;

#[derive(Debug)]
pub enum RegressionKind {
    StableToStable,
    StableToBeta,
    StableToNightly,
    Unknown,
}

impl ICE {
    pub fn test_regression_kind(self) -> Result<RegressionKind> {
        let stable_result = self.with_toolchain("stable").test()?;
        if stable_result.outcome() == Outcome::ICEd {
            return Ok(RegressionKind::StableToStable);
        }

        let ice = stable_result.ice.with_toolchain("beta");
        let beta_result = ice.test()?;

        if beta_result.outcome() == Outcome::ICEd {
            return Ok(RegressionKind::StableToBeta);
        }

        let ice = beta_result.ice.with_toolchain("nightly");
        let nightly_result = ice.test()?;

        if nightly_result.outcome() == Outcome::ICEd {
            return Ok(RegressionKind::StableToNightly);
        }

        Ok(RegressionKind::Unknown)
    }
}
