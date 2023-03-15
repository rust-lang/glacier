use glacier::rayon::iter::ParallelIterator;

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

    let mut untesteds =
        crate::github::get_labeled_issues(&config, "rust-lang/rust", "I-ICE".to_string()).unwrap();
    untesteds.retain(|i| !tested_issue_list.contains(&i.number));
    for untested in untesteds {
        let mut reproduction = Vec::new();
        let mut in_code_section = false;
        let mut in_code = false;
        let mut has_main = false;
        for line in untested.body.lines() {
            if in_code {
                if line.starts_with("```") {
                    in_code = false;
                    continue;
                }
                if line.starts_with("fn main") {
                    has_main = true;
                }
                reproduction.push(line);
            }
            if line.starts_with("### Code") {
                in_code_section = true;
            } else if line.starts_with('#') && in_code_section {
                in_code_section = false;
            }
            if (line.starts_with("```rust") || line.starts_with("```Rust")) && in_code_section {
                in_code = true;
            }
        }
        if reproduction.is_empty() {
            continue;
        }
        if !has_main {
            reproduction.push("");
            reproduction.push("fn main() {}");
        }
        std::fs::write(
            format!("./ices/{}.rs", untested.number),
            reproduction.join("\n"),
        )
        .unwrap();
    }

    let failed = glacier::test_all()
        .unwrap()
        .filter(|res| {
            if let Ok(test) = res {
                test.outcome() != glacier::Outcome::ICEd
            } else {
                true
            }
        })
        .collect::<Result<Vec<glacier::TestResult>, _>>()
        .unwrap();

    for result in &failed {
        std::fs::remove_file(result.path()).unwrap();
    }
}
