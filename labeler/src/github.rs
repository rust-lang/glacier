use once_cell::sync::Lazy;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env::{var, VarError};

static CLIENT: Lazy<Client> = Lazy::new(Client::new);

pub(crate) struct Config {
    token: String,
}

impl Config {
    pub(crate) fn from_env() -> Result<Self, VarError> {
        Ok(Self {
            token: var("LABEL_TOKEN")?,
        })
    }
}

pub(crate) fn label_issue(
    config: &Config,
    labels: &Labels,
    issue_number: usize,
) -> Result<(), reqwest::Error> {
    let url = format!(
        "https://api.github.com/repos/rust-lang/rust/issues/{}/labels",
        issue_number
    );

    CLIENT
        .post(&url)
        .bearer_auth(&config.token)
        .json(&labels)
        .send()?
        .error_for_status()?;

    Ok(())
}

pub(crate) fn get_labeled_issues(
    config: &Config,
    label_name: String,
) -> Result<Vec<Issue>, reqwest::Error> {
    let url = format!(
        "https://api.github.com/repos/rust-lang/rust/issues?labels={}",
        label_name
    );

    let issue: Vec<Issue> = CLIENT
        .get(&url)
        .bearer_auth(&config.token)
        .send()?
        .error_for_status()?
        .json()?;

    Ok(issue)
}

#[derive(Serialize, Debug)]
pub(crate) struct Labels {
    pub(crate) labels: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Issue {
    pub(crate) number: usize,
}
