use chrono::{Duration, Utc};

use crate::github::IssueState;

mod github;

fn main() {
    let config = github::Config::from_env(false).unwrap();

    let mut tested_issue_list = glacier::discover("./ices")
        .unwrap()
        .into_iter()
        .map(|ice| ice.id())
        .collect::<Vec<_>>();
    tested_issue_list.sort_unstable();
    tested_issue_list.dedup();
    println!("current tested issue list: {:?}", tested_issue_list);

    let issues =
        crate::github::get_labeled_issues(&config, "rust-lang/rust", "glacier".to_string());
    let mut labeled_issue_numbers: Vec<usize> = Vec::new();
    let mut closed_issue_numbers: Vec<usize> = Vec::new();
    for i in issues.unwrap() {
        let recently_closed = if let Some(closed_at) = i.closed_at {
            (Utc::now() - closed_at) < Duration::days(3)
        } else {
            false
        };
        if i.state == IssueState::Closed
            && !recently_closed
            && tested_issue_list.contains(&i.number)
        {
            closed_issue_numbers.push(i.number);
        }
        labeled_issue_numbers.push(i.number);
    }
    tested_issue_list.retain(|&x| !labeled_issue_numbers.contains(&x));
    println!("unlabeled issue list: {:?}", tested_issue_list);
    println!("closed issues list: {:?}", closed_issue_numbers);

    let labels: crate::github::Labels = crate::github::Labels {
        labels: vec!["glacier".to_string()],
    };
    for i in tested_issue_list {
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

    // Then we use `GITHUB_TOKEN` instead of `LABEL_TOKEN`.
    let config = github::Config::from_env(true).unwrap();
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
