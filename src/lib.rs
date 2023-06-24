use anyhow::{bail, ensure, Result};
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fmt};

pub use rayon;

static RUSTC: &str = "rustc";
static ICES_PATH: &str = "ices";
static SHELL: &str = "bash";

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum TestMode {
    SingleFile,
    ShellScript,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ICE {
    path: PathBuf,
    mode: TestMode,
}

impl ICE {
    fn from_path(path: PathBuf) -> Result<Self> {
        let mode = match path.extension().and_then(|e| e.to_str()) {
            Some("rs") => TestMode::SingleFile,
            Some("sh") => TestMode::ShellScript,
            _ => bail!("unknown ICE test extension: {}", path.display()),
        };

        Ok(Self { path, mode })
    }

    pub fn id(&self) -> IssueId {
        let s = self
            .path
            .file_stem()
            .unwrap()
            .to_owned()
            .into_string()
            .unwrap();
        // Some files have names like 123-1.rs; only get the first part of it
        let s = s.split('-').next().unwrap();
        IssueId(s.parse().unwrap())
    }

    fn test(self) -> Result<TestResult> {
        let workdir = tempfile::tempdir()?;
        let output = match self.mode {
            TestMode::SingleFile => Command::new(RUSTC)
                .args(["--edition", "2021"])
                .arg(std::fs::canonicalize(&self.path)?)
                .current_dir(workdir.path())
                .output()?,
            TestMode::ShellScript => Command::new(SHELL)
                .arg(std::fs::canonicalize(&self.path)?)
                .current_dir(workdir.path())
                .output()?,
        };

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        Ok(TestResult {
            ice: self,
            outcome: match output.status.code() {
                _ if stderr.contains("error: internal compiler error") => Outcome::ICEd,
                Some(0) => Outcome::NoError,
                Some(101) => Outcome::ICEd, // An ICE will cause an error code of 101
                // Bash uses 128+N for processes terminated by signal N
                Some(x) if x > 128 => Outcome::ICEd,
                Some(_) => Outcome::Errored,
                None => Outcome::ICEd, // If rustc receives a signal treat is as an ICE
            },
            stdout,
            stderr,
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct IssueId(pub usize);

/// Filters which ICEs should be tested.
#[derive(Default)]
pub struct Filter {
    ids: Vec<IssueId>,
}

impl Filter {
    pub fn try_from_args(args: std::env::Args) -> Result<Self> {
        let ids = args
            .skip(1)
            .map(|arg| Ok(IssueId(arg.parse()?)))
            .collect::<Result<_>>()?;
        Ok(Self { ids })
    }

    pub fn matches(&self, ice: &ICE) -> bool {
        self.ids.is_empty() || self.ids.contains(&ice.id())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Outcome {
    NoError,
    Errored,
    ICEd,
}

#[derive(Debug)]
pub struct TestResult {
    ice: ICE,
    outcome: Outcome,
    stdout: String,
    stderr: String,
}

impl TestResult {
    pub fn path(&self) -> &Path {
        &self.ice.path
    }

    pub fn outcome(&self) -> Outcome {
        self.outcome
    }

    pub fn stdout(&self) -> &str {
        &self.stdout
    }

    pub fn stderr(&self) -> &str {
        &self.stderr
    }

    pub fn title(&self) -> String {
        let path = self.path().display();

        match self.outcome {
            Outcome::NoError => format!("{path}: fixed with no errors"),
            Outcome::Errored => format!("{path}: fixed with errors"),
            Outcome::ICEd => format!("{path}: ICEd"),
        }
    }

    pub fn description(&self) -> Option<String> {
        if self.outcome != Outcome::ICEd {
            let mut out = String::new();
            out.push_str("=== stdout ===\n");
            out.push_str(self.stdout());
            out.push_str("=== stderr ===\n");
            out.push_str(self.stderr());
            out.push_str("==============");

            Some(out.replace('\0', "NUL"))
        } else {
            None
        }
    }

    pub fn outcome_token(&self) -> char {
        match self.outcome {
            Outcome::ICEd => '.',
            Outcome::Errored => 'E',
            Outcome::NoError => 'N',
        }
    }

    pub fn issue_url(&self) -> String {
        let file_name = self.path().file_name().unwrap().to_str().unwrap();

        let issue_number = file_name
            .split(|ch: char| !ch.is_ascii_digit())
            .next()
            .unwrap();

        format!("https://github.com/rust-lang/rust/issues/{issue_number}")
    }

    pub fn syntax(&self) -> &'static str {
        match self.ice.mode {
            TestMode::ShellScript => "bash",
            TestMode::SingleFile => "rust",
        }
    }
}

impl fmt::Display for TestResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.title())?;

        if let Some(description) = self.description() {
            write!(f, "\n{description}")?;
        }

        Ok(())
    }
}

pub fn discover(dir: &str) -> Result<Vec<ICE>> {
    let mut ices = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        // Ignore dotfiles
        if entry
            .path()
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .starts_with('.')
        {
            continue;
        }
        ices.push(ICE::from_path(entry.path())?);
    }

    ices.sort_unstable_by(|a, b| {
        alphanumeric_sort::compare_os_str(a.path.as_os_str(), b.path.as_os_str())
    });

    Ok(ices)
}

pub fn test_all_matching_filter(
    filter: &Filter,
) -> Result<impl IndexedParallelIterator<Item = Result<TestResult>>> {
    env::set_var("RUSTUP_TOOLCHAIN", "nightly");

    let output = Command::new("rustc").arg("--version").output()?;

    ensure!(
        output.status.success(),
        "nightly toolchain is not installed, run `rustup install nightly`"
    );
    let all_ices = discover(ICES_PATH)?;
    let ices_to_test: Vec<ICE> = all_ices
        .into_iter()
        .filter(|ice| filter.matches(ice))
        .collect();

    eprintln!(
        "running {} tests for {}",
        ices_to_test.len(),
        String::from_utf8_lossy(&output.stdout)
    );

    Ok(ices_to_test.into_par_iter().map(|ice| ice.test()))
}

pub fn test_all() -> Result<impl IndexedParallelIterator<Item = Result<TestResult>>> {
    test_all_matching_filter(&Filter::default())
}
