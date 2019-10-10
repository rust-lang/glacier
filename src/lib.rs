use rayon::prelude::*;
use std::error::Error;
use std::fmt;
use std::path::{Path, PathBuf};
use std::process::Command;

pub use rayon;

static RUSTC: &str = "rustc";
static ICES_PATH: &str = "ices";

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum TestMode {
    SingleFile,
    ShellScript,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ICE {
    path: PathBuf,
    mode: TestMode,
}

impl ICE {
    fn from_path(path: PathBuf) -> Result<Self, Box<dyn Error>> {
        let mode = match path.extension().and_then(|e| e.to_str()) {
            Some("rs") => TestMode::SingleFile,
            Some("sh") => TestMode::ShellScript,
            _ => return Err(format!("unknown ICE test extension: {}", path.display()).into()),
        };

        Ok(Self { path, mode })
    }

    fn test(self) -> Result<TestResult, Box<dyn Error>> {
        let workdir = tempfile::tempdir()?;
        let output = match self.mode {
            TestMode::SingleFile => Command::new(RUSTC)
                .arg(std::fs::canonicalize(&self.path)?)
                .current_dir(workdir.path())
                .output()?,
            TestMode::ShellScript => Command::new(std::fs::canonicalize(&self.path)?)
                .current_dir(workdir.path())
                .output()?,
        };
        Ok(TestResult {
            ice: self,
            outcome: match output.status.code() {
                Some(0) => Outcome::NoError,
                Some(101) => Outcome::ICEd, // An ICE will cause an error code of 101
                Some(_) => Outcome::Errored,
                None => Outcome::ICEd, // If rustc receives a signal treat is as an ICE
            },
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        })
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
            Outcome::NoError => format!("{}: fixed with no errors", path),
            Outcome::Errored => format!("{}: fixed with errors", path),
            Outcome::ICEd => format!("{}: ICEd", path),
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

            Some(out)
        } else {
            None
        }
    }

    pub fn issue_url(&self) -> Option<String> {
        let file_name = self.path().file_name()?.to_str()?;

        let issue_number = file_name.split(|ch: char| !ch.is_ascii_digit()).next()?;

        Some(format!(
            "https://github.com/rust-lang/rust/issues/{}",
            issue_number
        ))
    }
}

impl fmt::Display for TestResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.title())?;

        if let Some(description) = self.description() {
            write!(f, "\n{}", description)?;
        }

        Ok(())
    }
}

fn discover(dir: &str) -> Result<Vec<ICE>, Box<dyn Error>> {
    let mut ices = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        // Ignore dotfiles
        if entry
            .path()
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .starts_with(".")
        {
            continue;
        }
        ices.push(ICE::from_path(entry.path())?);
    }
    Ok(ices)
}

pub fn test_all() -> Result<impl IndexedParallelIterator<Item = Result<TestResult, String>>, String>
{
    let iter = discover(ICES_PATH)
        .map_err(|e| e.to_string())?
        .into_par_iter()
        .map(|ice| ice.test().map_err(|e| e.to_string()));

    Ok(iter)
}
