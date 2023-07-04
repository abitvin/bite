use std::str::Chars;

#[derive(Clone)]
pub struct CodeIter<'a> {
    code: Chars<'a>,
    column: usize,
    line: usize,
}

impl<'a> CodeIter<'a> {
    pub fn new(code: &'a str) -> Self {
        Self {
            code: code.chars(),
            column: 0,
            line: 0,
        }
    }
}

impl<'a> Iterator for CodeIter<'a> {
    type Item = char;
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.code.next() {
            Some(c) => {
                if c == '\n' {
                    self.column = 0;
                    self.line += 1;
                }
                else {
                    self.column += 1;
                }

                Some(c)
            },
            None => None,
        }
    }
}