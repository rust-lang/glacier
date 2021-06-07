use crate::github::IssueState;
use regex::Regex;
use std::fs;

mod github;

fn issue_list() -> Vec<usize> {
    let mut issue_list = Vec::new();
    for path in fs::read_dir("./ices").unwrap() {
        let file_name = path.unwrap().path().display().to_string();
        let re = Regex::new(r"[0-9]+").unwrap();
        let caps = re.captures(&file_name).unwrap();
        issue_list.push(caps.get(0).unwrap().as_str().parse().unwrap());
    }
    issue_list.sort_unstable();
    issue_list.dedup();

    issue_list
}

fn main() {
    let config = github::Config::from_env().unwrap();

    let mut unlabeled_issue_list = issue_list();
    println!("current tested issue list: {:?}", unlabeled_issue_list);

    let issues =
        crate::github::get_labeled_issues(&config, "rust-lang/rust", "glacier".to_string());
    let mut labeled_issue_numbers: Vec<usize> = Vec::new();
    let mut closed_issue_numbers: Vec<usize> = Vec::new();
    for i in issues.unwrap() {
        if i.state == IssueState::Closed {
            closed_issue_numbers.push(i.number);
        }
        labeled_issue_numbers.push(i.number);
    }
    unlabeled_issue_list.retain(|&x| !labeled_issue_numbers.contains(&x));
    println!("unlabeled issue list: {:?}", unlabeled_issue_list);
    println!("closed issues list: {:?}", closed_issue_numbers);

    let labels: crate::github::Labels = crate::github::Labels {
        labels: vec!["glacier".to_string()],
    };
    for i in unlabeled_issue_list {
        println!(
            "Adding the `{:?}` label to issue#{:?}...",
            &labels.labels, i
        );
        let res = crate::github::label_issue(&config, &labels, i);
        match res {
            Ok(_) => {
                println!("Added the `{:?}` label to issue#{:?}", &labels.labels, i);
            }
            Err(e) => {
                eprintln!("Failed the `{:?}` label to issue#{:?}", &labels.labels, i);
                eprintln!("The reason is here: {:?}", e);
                std::process::exit(1);
            }
        }
    }

    let issues_in_triage =
        crate::github::get_labeled_issues(&config, "rust-lang/glacier", "triage".to_string())
            .unwrap();
    for id in closed_issue_numbers {
        let title = format!("issue-{}", id);
        if issues_in_triage
            .iter()
            .any(|issue| issue.title.starts_with(&title))
        {
            // Triage issue already created
            continue;
        }

        let body = format!("See rust-lang/rust#{}", id);

        crate::github::create_issue(
            &config,
            &format!("{} has been closed", title),
            &body,
            &["triage"],
        )
        .unwrap()
    }
}
