use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::blocking::Client;
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

    let mut issues: Vec<Issue> = CLIENT
        .get(&url)
        .bearer_auth(&config.token)
        .send()?
        .error_for_status()?
        .json()?;

    let pages = get_result_length(&config, &url).unwrap();

    if pages > 1 {
        for i in 2..=pages {
            let url = format!(
                "https://api.github.com/repos/rust-lang/rust/issues?labels={}&page={}",
                label_name, i
            );
            let mut paged_issues: Vec<Issue> = CLIENT
                .get(&url)
                .bearer_auth(&config.token)
                .send()?
                .error_for_status()?
                .json()?;

            issues.append(&mut paged_issues);
        }
    }

    Ok(issues)
}

fn get_result_length(config: &Config, url: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let res = CLIENT.get(url).bearer_auth(&config.token).send()?;

    if res.status().is_success() {
        if let Some(link) = res.headers().get("Link") {
            let link_string = String::from_utf8(link.as_bytes().to_vec()).unwrap();
            let re_last_page = Regex::new(r#"page=[0-9]+>; rel="last""#).unwrap();
            let re_page_number = Regex::new(r"[0-9]+").unwrap();
            let last_page = re_last_page
                .captures(&link_string)
                .unwrap()
                .get(0)
                .unwrap()
                .as_str();
            let page_number = re_page_number.captures(&last_page).unwrap();
            let pages: usize = page_number.get(0).unwrap().as_str().parse().unwrap();

            Ok(pages)
        } else {
            Ok(0)
        }
    } else {
        Ok(0)
    }
}

#[derive(Serialize, Debug)]
pub(crate) struct Labels {
    pub(crate) labels: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Issue {
    pub(crate) number: usize,
}
