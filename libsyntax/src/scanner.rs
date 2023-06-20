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
        value.chars()
            .zip(&mut self.code)
            .fold(Some(String::new()), |acc, (a, b)| {
                acc.filter(|_| a == b).map(|s| s + &String::from(a))
            })
    }

    pub fn scan_alphabetic(&mut self) -> Option<char> {
        self.code.next().filter(|x| x.is_alphabetic())
    }

    pub fn scan_alphanumeric(&mut self) -> Option<char> {
        self.code.next().filter(|x| x.is_alphabetic() || x.is_digit(10))
    }

    pub fn scan_digit(&mut self) -> Option<char> {
        self.code.next().filter(|x| x.is_digit(10))
    }
}