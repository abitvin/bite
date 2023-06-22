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

            match c1 {
                Some(c1) if c0 == c1 => continue,
                _ => return None,
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
        let mut s = String::new();
        let mut chars = self.code.clone();

        loop {
            let c = chars.next();

            match c {
                Some(c) if c.is_digit(10) => s += &String::from(c),
                _ => break,
            }
        }

        if s.len() == 0 {
            return None;
        }
        
        self.code = chars;
        Some(s)
    }

    pub fn skip(&mut self, char: char) -> bool {
        self.skip_when(|c| c == char)
    }

    pub fn skip_newline(&mut self) -> bool {
        match self.skip_range(0, 1, |c| c == '\r') {
            1 => self.skip_range(0, 1, |c| c == '\n') <= 1,
            _ => self.skip_range(0, 1, |c| c == '\n') == 1 && self.skip_range(0, 1, |c| c == '\r') == 0,
        }
    }

    pub fn skip_spaces(&mut self) -> bool {
        self.skip_when(|c| c == ' ')
    }

    pub fn skip_whitespaces(&mut self) -> bool {
        self.skip_when(|c| c == ' ' || c == '\n' || c == '\r')
    }

    fn step_when(&mut self, condition: impl Fn(char) -> bool) -> Option<char> {
        let peek = self.code.clone().peekable().next()?;

        match condition(peek) {
            true => self.code.next(),
            false => None,
        }
    }

    fn skip_range(&mut self, min: usize, max: usize, condition: impl Fn(char) -> bool) -> usize {
        let chars = self.code.clone();
        let mut found = 0;

        for c in chars {
            if !condition(c) {
                break;
            }

            found += 1;

            if found == max {
                break;
            }
        }

        if found < min || found > max {
            return 0;
        }

        if found > 0 {
            self.code.nth(found - 1);
        }

        found
    }

    fn skip_when(&mut self, condition: impl Fn(char) -> bool) -> bool {
        let chars = self.code.clone();
        let mut found = false;

        for c in chars {
            if !condition(c) {
                break;
            }

            self.code.next();
            found = true;
        }

        found
    }
}