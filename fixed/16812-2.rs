use std::path::Path;
use std::path::PathBuf;

pub enum Source
{
    File(Path), // compiler error :(
    //File(PathBuf), // works :)
}

pub fn source_to_dbtype(src: &Source) -> u8
{
    match src
    {
        &Source::File(..) => 1,
    }
}

fn main() {}
