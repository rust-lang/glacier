use glacier::RegressionKind;
use std::collections::HashMap;

mod github;

fn main() {
    let config = github::Config::from_env().unwrap();

    let issues = glacier::discover("./ices").unwrap();
    let mut issues = issues
        .into_iter()
        .map(|ice| (ice.id(), ice))
        .collect::<HashMap<_, _>>();
    let mut unlabeled_issue_list = issues.keys().copied().collect::<Vec<_>>();
    println!("current tested issue list: {:?}", unlabeled_issue_list);

    let github_issues = crate::github::get_labeled_issues(&config, "glacier".to_string()).unwrap();
    let mut labeled_issue_numbers: Vec<usize> = Vec::new();
    let mut regression_triage: Vec<usize> = Vec::new();
    for i in github_issues {
        if !i.labels.iter().any(|label| {
            [
                "regression-from-stable-to-stable",
                "regression-from-stable-to-beta",
                "regression-from-stable-to-nightly",
                "regression-untriaged",
            ]
            .contains(&label.name.as_str())
        }) {
            regression_triage.push(i.number);
        }
        labeled_issue_numbers.push(i.number);
    }
    unlabeled_issue_list.retain(|&x| !labeled_issue_numbers.contains(&x));
    println!("unlabeled issue list: {:?}", unlabeled_issue_list);
    println!("regression triage list: {:?}", regression_triage);

    let labels: crate::github::Labels = crate::github::Labels {
        labels: vec!["glacier".to_string()],
    };

    for i in unlabeled_issue_list {
        println!(
            "Adding the `{:?}` label to issue#{:?}...",
            &labels.labels, i
        );
        label_issue(&config, &labels, i);
    }
    for i in regression_triage {
        println!("Regression testing issue#{:?}...", i);
        let ice = issues.remove(&i).unwrap();
        let label = match ice.test_regression_kind().unwrap() {
            RegressionKind::StableToStable => "regression-from-stable-to-stable",
            RegressionKind::StableToBeta => "regression-from-stable-to-beta",
            RegressionKind::StableToNightly => "regression-from-stable-to-nightly",
            RegressionKind::Unknown => "regression-untriaged",
        };
        let labels: crate::github::Labels = crate::github::Labels {
            labels: vec![label.to_string()],
        };
        println!("Adding the `{:?}` label to issue#{:?}...", label, i);
        label_issue(&config, &labels, i);
    }
}

fn label_issue(config: &github::Config, labels: &crate::github::Labels, issue: usize) {
    match crate::github::label_issue(&config, &labels, issue) {
        Ok(_) => {
            println!(
                "Added the `{:?}` label to issue#{:?}",
                &labels.labels, issue
            );
        }
        Err(e) => {
            eprintln!(
                "Failed the `{:?}` label to issue#{:?}",
                &labels.labels, issue
            );
            eprintln!("The reason is here: {:?}", e);
            std::process::exit(1);
        }
    }
}
