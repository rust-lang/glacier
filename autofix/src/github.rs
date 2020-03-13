use anyhow::Result;
use once_cell::sync::Lazy;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::env::var;

static CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .user_agent("rust-lang/glacier")
        .build()
        .unwrap()
});

pub(crate) struct Config {
    token: String,
    repo: String,
}

impl Config {
    pub(crate) fn from_env() -> Result<Self> {
        Ok(Self {
            token: var("GITHUB_TOKEN")?,
            repo: var("GITHUB_REPOSITORY")?,
        })
    }

    pub(crate) fn remote_url(&self) -> String {
        format!(
            "https://x-access-token:{}@github.com/{}",
            self.token, self.repo
        )
    }
}

/// https://developer.github.com/v3/pulls/#input
#[derive(Serialize)]
pub(crate) struct PullRequestOptions<'a> {
    pub(crate) title: &'a str,
    pub(crate) body: &'a str,
    pub(crate) head: &'a str,
    pub(crate) base: &'a str,
}

#[derive(Deserialize)]
struct PullRequest {
    html_url: String,
}

pub(crate) fn pull_request(config: &Config, options: &PullRequestOptions) -> Result<String> {
    let url = format!("https://api.github.com/repos/{}/pulls", config.repo);

    let pr: PullRequest = CLIENT
        .post(&url)
        .header(reqwest::header::ACCEPT, "application/vnd.github.v3+json")
        .bearer_auth(&config.token)
        .json(&options)
        .send()?
        .error_for_status()?
        .json()?;

    Ok(pr.html_url)
}
