use crate::json::JSON;
use crate::lexer::{Lexer, Token, TokenVal};

use std::collections::HashMap;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub struct ParseError {
    err_msg: String,
    line_no: u64,
}

impl ParseError {
    fn new(err_msg: String, line_no: u64) -> Self {
        ParseError { err_msg, line_no }
    }
}

pub type ParseResult = Result<JSON, ParseError>;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    curr_token: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let lexer = Lexer::new(input);

        Parser {
            lexer,
            curr_token: None,
        }
    }

    fn cont(&mut self) -> Result<(), ParseError> {
        match self.lexer.next_token() {
            Ok(token) => {
                self.curr_token = Some(token);
                Ok(())
            }
            Err(err) => self.error(err.err_msg, err.line_no),
        }
    }

    fn matches(&mut self, value: TokenVal) -> Result<bool, ParseError> {
        let token = self.curr_token.take().unwrap();

        if token.value != value {
            self.curr_token = Some(token);
            Ok(false)
        } else {
            self.cont()?;
            Ok(true)
        }
    }

    fn expect(&mut self, value: TokenVal, err_msg: String) -> Result<(), ParseError> {
        let token = self.curr_token.take().unwrap();

        if token.value != value {
            self.error(err_msg, token.line_no)
        } else {
            self.cont()
        }
    }

    fn error(&self, err_msg: String, line_no: u64) -> Result<(), ParseError> {
        Err(ParseError::new(err_msg, line_no))
    }

    fn throw(&self, err_msg: String, line_no: u64) -> ParseResult {
        Err(ParseError::new(err_msg, line_no))
    }

    fn get_string(&mut self) -> Result<String, ParseError> {
        let token = self.curr_token.take().unwrap();
        self.cont()?;

        match token.value {
            TokenVal::JString(x) => Ok(x),
            _ => Err(ParseError::new(
                "Expecting string".to_string(),
                token.line_no,
            )),
        }
    }

    fn parse_object(&mut self) -> ParseResult {
        let mut obj = HashMap::new();

        if self.matches(TokenVal::RBrace)? {
            return Ok(JSON::JSONObject(obj));
        }

        let key = self.get_string()?;
        self.expect(TokenVal::Colon, "Expecting colon after key".to_string())?;
        let val = self.parse_elem()?;
        obj.insert(key, val);

        while self.matches(TokenVal::Comma)? {
            let key = self.get_string()?;
            self.expect(TokenVal::Colon, "Expecting colon after key".to_string())?;
            let val = self.parse_elem()?;
            obj.insert(key, val);
        }

        self.expect(
            TokenVal::RBrace,
            "Expecting right brace at end of object".to_string(),
        )?;

        Ok(JSON::JSONObject(obj))
    }

    fn parse_array(&mut self) -> ParseResult {
        let mut arr = Vec::new();

        if self.matches(TokenVal::RBrack)? {
            return Ok(JSON::JSONArray(arr));
        }

        let elem = self.parse_elem()?;
        arr.push(elem);

        while self.matches(TokenVal::Comma)? {
            let elem = self.parse_elem()?;
            arr.push(elem);
        }

        self.expect(
            TokenVal::RBrack,
            "Expecting right bracket at end of array".to_string(),
        )?;

        Ok(JSON::JSONArray(arr))
    }

    fn parse_elem(&mut self) -> ParseResult {
        let token = self.curr_token.take().unwrap();
        self.cont()?;

        match token.value {
            TokenVal::LBrace => self.parse_object(),
            TokenVal::LBrack => self.parse_array(),
            TokenVal::True => Ok(JSON::JSONBool(true)),
            TokenVal::False => Ok(JSON::JSONBool(false)),
            TokenVal::Null => Ok(JSON::JSONNull),
            TokenVal::JString(x) => Ok(JSON::JSONString(x)),
            TokenVal::JNumber(x) => Ok(JSON::JSONNum(x)),
            _ => self.throw("Unexpected token".to_string(), token.line_no),
        }
    }

    pub fn parse(&mut self) -> ParseResult {
        self.cont()?;

        let elem = self.parse_elem()?;

        self.expect(TokenVal::Eof, "Expecting EOF".to_string())?;

        Ok(elem)
    }
}
