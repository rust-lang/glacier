use std::convert::{TryFrom, TryInto};
use std::io;

pub struct MyStream;
pub struct OtherStream;

pub async fn connect() -> io::Result<MyStream> {
    let stream: MyStream = OtherStream.try_into()?;
    Ok(stream)
}

impl TryFrom<OtherStream> for MyStream {}

fn main() {}
