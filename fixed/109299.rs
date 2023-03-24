trait Document {
    fn cursor(&self) -> Lexer::Cursor<'_>;
}

struct Lexer<'d> {
    cursor: (),
    _phantom: std::marker::PhantomData<&'d ()>,
}

impl<'cursor> Lexer<'d> {
    type Cursor<'a> = DocCursorImpl<'a>;
}

fn main() {}
