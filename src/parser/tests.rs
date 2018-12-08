use crate::json::JSON;
use crate::parser::{ParseError, Parser};

use std::collections::HashMap;

#[test]
fn test_false() {
    let mut parser = Parser::new("false");
    let actual = parser.parse().unwrap();
    let expected = JSON::JSONBool(false);

    assert_eq!(actual, expected);
}

#[test]
fn test_true() {
    let mut parser = Parser::new("true");
    let actual = parser.parse().unwrap();
    let expected = JSON::JSONBool(true);

    assert_eq!(actual, expected);
}

#[test]
fn test_null() {
    let mut parser = Parser::new("null");
    let actual = parser.parse().unwrap();
    let expected = JSON::JSONNull;

    assert_eq!(actual, expected);
}

#[test]
fn test_simple_float() {
    let mut parser = Parser::new("44.3");
    let actual = parser.parse().unwrap();
    let expected = JSON::JSONNum(44.3);

    assert_eq!(actual, expected);
}

#[test]
fn test_string() {
    let mut parser = Parser::new("\"Hello\"");
    let actual = parser.parse().unwrap();
    let expected = JSON::JSONString("Hello".to_string());

    assert_eq!(actual, expected);
}

#[test]
fn test_simple_array() {
    let mut parser = Parser::new("[true, false, null, 1.2]");
    let actual = parser.parse().unwrap();
    let expected = JSON::JSONArray(vec![
        JSON::JSONBool(true),
        JSON::JSONBool(false),
        JSON::JSONNull,
        JSON::JSONNum(1.2),
    ]);

    assert_eq!(actual, expected);
}

#[test]
fn test_arr_missing_comma() {
    let mut parser = Parser::new("[true, false null, 1.2]");
    let actual = parser.parse();
    let expected = Err(ParseError::new(
        "Expecting right bracket at end of array".to_string(),
        1,
    ));

    assert_eq!(actual, expected);
}

#[test]
fn test_simple_obj() {
    let mut parser = Parser::new("{ \"abc\":1.1, \"def\":2.2, \"xyz\":3.3 }");
    let actual = parser.parse().unwrap();

    let mut hmap = HashMap::new();
    hmap.insert("abc".to_string(), JSON::JSONNum(1.1));
    hmap.insert("def".to_string(), JSON::JSONNum(2.2));
    hmap.insert("xyz".to_string(), JSON::JSONNum(3.3));

    let expected = JSON::JSONObject(hmap);

    assert_eq!(actual, expected);
}

#[test]
fn test_extra_comma() {
    let mut parser = Parser::new("{ \"abc\":1.1, \"def\":2.2, \"xyz\":3.3, }");
    let actual = parser.parse();
    let expected = Err(ParseError::new("Expecting string".to_string(), 1));

    assert_eq!(actual, expected);
}

#[test]
fn test_obj_missing_colon() {
    let mut parser = Parser::new("{ \"abc\" 1.1, \"def\":2.2, \"xyz\":3.3 }");
    let actual = parser.parse();
    let expected = Err(ParseError::new("Expecting colon after key".to_string(), 1));

    assert_eq!(actual, expected);
}

#[test]
fn test_invalid_obj_key() {
    let mut parser = Parser::new("{ 15 : false }");
    let actual = parser.parse();
    let expected = Err(ParseError::new("Expecting string".to_string(), 1));

    assert_eq!(actual, expected);
}
