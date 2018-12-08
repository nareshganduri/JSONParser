use crate::lexer::{LexError, Lexer, TokenVal};

#[test]
fn test_eof() {
    let mut lexer = Lexer::new("");
    let actual = lexer.next_token().unwrap();
    let expected = TokenVal::Eof;

    assert_eq!(actual.value, expected);
}

#[test]
fn test_puncts() {
    let mut lexer = Lexer::new("{}[]:,");

    let actual = lexer.next_token().unwrap();
    let expected = TokenVal::LBrace;
    assert_eq!(actual.value, expected);

    let actual = lexer.next_token().unwrap();
    let expected = TokenVal::RBrace;
    assert_eq!(actual.value, expected);

    let actual = lexer.next_token().unwrap();
    let expected = TokenVal::LBrack;
    assert_eq!(actual.value, expected);

    let actual = lexer.next_token().unwrap();
    let expected = TokenVal::RBrack;
    assert_eq!(actual.value, expected);

    let actual = lexer.next_token().unwrap();
    let expected = TokenVal::Colon;
    assert_eq!(actual.value, expected);

    let actual = lexer.next_token().unwrap();
    let expected = TokenVal::Comma;
    assert_eq!(actual.value, expected);
}

#[test]
fn test_null() {
    let mut lexer = Lexer::new("null");
    let actual = lexer.next_token().unwrap();
    let expected = TokenVal::Null;

    assert_eq!(actual.value, expected);
}

#[test]
fn test_string() {
    let mut lexer = Lexer::new("\"hello world\"");
    let actual = lexer.next_token().unwrap();
    let expected = TokenVal::JString("hello world".to_string());

    assert_eq!(actual.value, expected);
}

#[test]
fn test_string_eof() {
    let mut lexer = Lexer::new("\"hello world");
    let actual = lexer.next_token();
    let expected = Err(LexError::new(
        "Unexpected EOF while parsing string".to_string(),
        1,
    ));

    assert_eq!(actual, expected);
}

#[test]
fn test_simple_float() {
    let mut lexer = Lexer::new("4.5");
    let actual = lexer.next_token().unwrap();
    let expected = TokenVal::JNumber(4.5);

    assert_eq!(actual.value, expected);
}

#[test]
fn test_complex_float() {
    let mut lexer = Lexer::new("-3.775e+2");
    let actual = lexer.next_token().unwrap();
    let expected = TokenVal::JNumber(-377.5);

    assert_eq!(actual.value, expected);
}

#[test]
fn test_bad_float() {
    let mut lexer = Lexer::new("33.");
    let actual = lexer.next_token();
    let expected = Err(LexError::new("Need at least one digit".to_string(), 1));

    assert_eq!(actual, expected);
}

#[test]
fn test_bools() {
    let mut lexer = Lexer::new("true false");

    let actual = lexer.next_token().unwrap();
    let expected = TokenVal::True;
    assert_eq!(actual.value, expected);

    let actual = lexer.next_token().unwrap();
    let expected = TokenVal::False;
    assert_eq!(actual.value, expected);
}

#[test]
fn test_invalid_keyword() {
    let mut lexer = Lexer::new("hello");
    let actual = lexer.next_token();
    let expected = Err(LexError::new("Invalid character".to_string(), 1));

    assert_eq!(actual, expected);
}

#[test]
fn test_spaces() {
    let mut lexer = Lexer::new("      ");
    let actual = lexer.next_token().unwrap();
    let expected = TokenVal::Eof;

    assert_eq!(actual.value, expected);
}

#[test]
fn test_line_no() {
    let mut lexer = Lexer::new("\n\ntrue");
    let actual = lexer.next_token().unwrap();
    let expected = 3;

    assert_eq!(actual.line_no, expected);
}
