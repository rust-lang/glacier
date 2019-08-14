use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::Command;

static RUSTC: &str = "rustc";

enum TestMode {
    SingleFile,
    ShellScript,
}

pub(crate) struct ICE {
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

    pub(crate) fn path(&self) -> &Path {
        &self.path
    }

    pub(crate) fn test(&self) -> Result<TestResult, Box<dyn Error>> {
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

#[derive(Copy, Clone, PartialEq, Eq)]
pub(crate) enum Outcome {
    NoError,
    Errored,
    ICEd,
}

pub(crate) struct TestResult {
    outcome: Outcome,
    stdout: String,
    stderr: String,
}

impl TestResult {
    pub(crate) fn outcome(&self) -> Outcome {
        self.outcome
    }

    pub(crate) fn stdout(&self) -> &str {
        &self.stdout
    }

    pub(crate) fn stderr(&self) -> &str {
        &self.stderr
    }
}

pub(crate) fn discover(dir: &Path) -> Result<Vec<ICE>, Box<dyn Error>> {
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
