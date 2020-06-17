pub struct Lint {}

impl Lint {}

/// Gathers all files in `src/clippy_lints` and gathers all lints inside
pub fn gather_all() -> impl Iterator<Item = Lint> {
    lint_files().flat_map(|f| gather_from_file(&f))
}

fn gather_from_file(dir_entry: &walkdir::DirEntry) -> impl Iterator<Item = Lint> {
    unimplemented!()
}

/// Collects all .rs files in the `clippy_lints/src` directory
fn lint_files() -> impl Iterator<Item = walkdir::DirEntry> {
    unimplemented!()
}

pub fn main() {}
