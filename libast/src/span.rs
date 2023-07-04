#[derive(Debug, PartialEq)]
pub struct Cursor {
    column: usize,
    line: usize,
    offset: usize,
}

impl Cursor {
    pub fn new(column: usize, line: usize, offset: usize) -> Self {
        Cursor { column, line, offset }
    }

    pub fn span_to(self, other: Self) -> Span {
        let width = other.offset - self.offset;
        Span::from_cursor(self, width)
    }
}

#[derive(Debug, PartialEq)]
pub struct Span {
    cursor: Cursor,
    width: usize,
}

impl Span {
    pub fn new(column: usize, line: usize, offset: usize, width: usize) -> Self {
        Self { cursor: Cursor { column, line, offset }, width }
    }

    pub fn from_cursor(cursor: Cursor, width: usize) -> Self {
        Self { cursor, width }
    }
}