use std::str::Chars;

pub struct Scanner<'a> {
    code: Chars<'a>,
}

impl<'a> Scanner<'a> {
    pub fn new(code: &'a str) -> Self {
        Self {
            code: code.chars()
        }
    }

    pub fn scan(&mut self, value: &str) -> Option<String> {
        let mut iter = self.code.clone();
        let chars = value.chars();
        
        for c0 in chars {
            let c1 = iter.next();

            if let Some(c1) = c1 {
                if c0 != c1 {
                    return None;
                }
            }
            else {
                return None;
            }
        }
        
        self.code = iter;
        Some(String::from(value))
    }

    pub fn scan_alphabetic(&mut self) -> Option<char> {
        self.step_when(|c| c.is_alphabetic())
    }

    pub fn scan_alphanumeric(&mut self) -> Option<char> {
        self.step_when(|c| c.is_alphabetic() || c.is_digit(10))
    }

    pub fn scan_digit(&mut self) -> Option<char> {
        self.step_when(|c| c.is_digit(10))
    }

    pub fn scan_digits(&mut self) -> Option<String> {
        let chars = self.code.clone();

        // for c in chars {

        // }

        unimplemented!()
    }

    fn is_eof(&self) -> bool {
        self.code.clone().next().is_none()
    }

    fn step_when(&mut self, condition: impl Fn(char) -> bool) -> Option<char> {
        let peek = self.code.clone().peekable().next()?;

        match condition(peek) {
            true => self.code.next(),
            false => None,
        }
    }
}