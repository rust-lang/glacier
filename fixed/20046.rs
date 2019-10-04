#[derive(PartialEq)]
enum IoErrorKind { BrokenPipe, XXX }
struct IoError {
    pub kind: IoErrorKind,
    pub detail: Option<String>
}
fn main() {
    let e: Result<u8, _> = Err(IoError{ kind: IoErrorKind::XXX, detail: None });
    match e {
        Ok(_) => true,
        Err(ref e) if e.kind == IoErrorKind::BrokenPipe => return,
        Err(IoError { kind: IoErrorKind::BrokenPipe, ..}) => return,
        Err(err) => panic!(err)
    };
}
