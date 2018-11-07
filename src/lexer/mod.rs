use std::str::Chars;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenVal {
    Eof,
    LBrace,
    RBrace,
    LBrack,
    RBrack,
    Comma,
    Colon,
    True,
    False,
    Null,
    JString(String),
    JNumber(f64)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub value: TokenVal,
    pub line_no: u64
}

#[derive(Debug, Clone, PartialEq)]
pub struct LexError {
    pub err_msg: String,
    pub line_no: u64
}

impl LexError {
    fn new(err_msg: String, line_no: u64) -> LexError {
        LexError {
            err_msg,
            line_no
        }
    }
}

type LexResult = Result<Token, LexError>;

pub struct Lexer<'a> {
    input: Chars<'a>,
    curr_char: Option<char>,
    line_no: u64
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut iter = input.chars();
        let c = iter.next();

        Lexer {
            input: iter,
            curr_char: c,
            line_no: 1
        }
    }

    fn new_token(&self, value: TokenVal) -> Token {
        Token {
            value: value,
            line_no: self.line_no
        }
    }

    fn error(&self, err_msg: String) -> Result<(), LexError> {
        Err(LexError::new(
            err_msg,
            self.line_no
        ))
    }

    fn throw(&self, err_msg: String) -> LexResult {
        let error = self.error(err_msg);
        match error {
            Err(x) => Err(x),
            _      => panic!("Lexer::error returned Ok()")
        }
    }

    fn peek(&self) -> Option<char> {
        self.curr_char
    }

    fn cont(&mut self) {
        if let Some(c) = self.peek() {
            if c == '\n' {
                self.line_no += 1;
            }
        }

        self.curr_char = self.input.next();
    }

    fn eat(&mut self, string: &mut String) {
        let c = self.peek();

        match c {
            None => return,
            Some(x) => {
                string.push(x);
                self.cont();
            }
        }
    }

    fn match_any(&self, chars: &str) -> bool {
        let c = self.peek();

        if let Some(c) = c {
            for ch in chars.chars() {
                if c == ch {
                    return true;
                }
            }

            false
        } else {
            false
        }
    }

    fn skip_spaces(&mut self) {
        while self.match_any(" \t\n\r") {
            self.cont();
        }
    }

    fn get_punct(&mut self) -> LexResult {
        let c = self.peek().unwrap();
        self.cont();

        match c {
            '{' => Ok(self.new_token(TokenVal::LBrace)),
            '}' => Ok(self.new_token(TokenVal::RBrace)),
            '[' => Ok(self.new_token(TokenVal::LBrack)),
            ']' => Ok(self.new_token(TokenVal::RBrack)),
            ':' => Ok(self.new_token(TokenVal::Colon)),
            ',' => Ok(self.new_token(TokenVal::Comma)),
            _   => self.throw("Invalid punctuation".to_string())
        }
    }

    fn get_escape_char(&mut self, string: &mut String) -> Result<(), LexError> {
        self.cont(); // eat the escape char

        let err_msg = "Invalid escape character".to_string();

        let c = self.peek();
        if let Some(c) = c {
            self.cont();

            match c {
                '\\' => string.push('\\'),
                '\"' => string.push('\"'),
                '/'  => string.push('/'),
                'b'  => string.push(0x8 as char),
                'f'  => string.push(0xc as char),
                'n'  => string.push('\n'),
                'r'  => string.push('\r'),
                't'  => string.push('\t'),
                'u'  => {
                    let mut code_pt = String::new();

                    for _ in 0..4 {
                        if !self.match_any("0123456789abcdefABCDEF") {
                            self.error(err_msg.clone())?;
                        } else {
                            code_pt.push(self.peek().unwrap());
                            self.cont();
                        }
                    }

                    let num: Result<u16, std::num::ParseIntError> 
                        = u16::from_str_radix(&code_pt, 16);
                    match num {
                        Ok(x) => {
                            let code_pt = std::char::from_u32(x as u32);
                            if let Some(x) = code_pt {
                                string.push(x);
                            } else {
                                self.error(err_msg)?;
                            }
                        },
                        Err(_) => self.error(err_msg)?
                    }
                },
                _    => self.error(err_msg)?
            }

            Ok(())
        } else {
            Ok(self.error(err_msg)?)
        }
    }

    fn get_string(&mut self) -> LexResult {
        self.cont(); // eat the opening quote

        let mut val = String::new();

        while let Some(c) = self.peek() {
            match c {
                '\x00' ... '\x1f' => self.error(
                    format!("Invalid character in string: {}", c as u8))?,
                '\\' => self.get_escape_char(&mut val)?,
                '\"' => {
                    self.cont();

                    return Ok(self.new_token(TokenVal::JString(val)));
                }
                _    => self.eat(&mut val)
            }
        }

        self.throw("Unexpected EOF while parsing string".to_string())
    }

    fn match_letters(&mut self, letters: &str, value: TokenVal) -> LexResult {
        let err = self.error("Invalid keyword".to_string());

        for x in letters.chars() {
            let c = self.peek();

            if let Some(c) = c {
                if c != x {
                    err?;
                    break;
                } else {
                    self.cont();
                }
            } else {
                err?;
                break;
            }
        }

        Ok(self.new_token(value))
    }

    fn get_keyword(&mut self) -> LexResult {
        let c = self.peek().unwrap();
        self.cont();

        match c {
            't' => self.match_letters("rue", TokenVal::True),
            'f' => self.match_letters("alse", TokenVal::False),
            'n' => self.match_letters("ull", TokenVal::Null),
            _   => self.throw("Invalid keyword".to_string())
        }
    }

    fn need_digit(&mut self, string: &mut String) -> Result<(), LexError> {
        if !self.match_any("0123456789") {
            self.error("Need at least one digit".to_string())?;
        }

        while self.match_any("0123456789") {
             self.eat(string);
        }

        Ok(())
    }

    fn get_number(&mut self) -> LexResult {
        let mut val = String::new();

        if self.peek() == Some('-') {
            self.eat(&mut val);
        }

        if self.peek() == Some('0') {
            self.eat(&mut val);
        } else if self.match_any("123456789") {
            self.eat(&mut val);

            while self.match_any("0123456789") {
                self.eat(&mut val);
            }
        } else {
            return self.throw("Invalid number".to_string());
        }

        if self.peek() == Some('.') {
            self.eat(&mut val);

            self.need_digit(&mut val)?;
        }
        
        if self.match_any("eE") {
            self.eat(&mut val);

            if self.match_any("-+") {
                self.eat(&mut val);
            }
            
            self.need_digit(&mut val)?;
        }

        let val: Result<f64, std::num::ParseFloatError> = val.parse();
        match val {
            Ok(val) => Ok(self.new_token(TokenVal::JNumber(val))),
            Err(_) => self.throw("Invalid floating-point literal".to_string())
        }
    }

    pub fn next_token(&mut self) -> LexResult {
        self.skip_spaces();

        if let Some(c) = self.peek() {
            if self.match_any("{}[],:") {
                self.get_punct()
            } else if self.match_any("tfn") {
                self.get_keyword()
            } else if self.match_any("-0123456789") {
                self.get_number()
            } else if c == '\"' {
                self.get_string()
            } else {
                self.throw("Invalid character".to_string())
            }
        } else {
            Ok(self.new_token(TokenVal::Eof))
        }
    }
}