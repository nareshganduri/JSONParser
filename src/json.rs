use std::collections::HashMap;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum JSON {
    JSONNum(f64),
    JSONString(String),
    JSONNull,
    JSONBool(bool),
    JSONArray(Vec<JSON>),
    JSONObject(HashMap<String, JSON>),
}

impl fmt::Display for JSON {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &JSON::JSONNum(ref x) => write!(f, "{}", x),
            &JSON::JSONString(ref x) => write!(f, "\"{}\"", x),
            &JSON::JSONNull => write!(f, "null"),
            &JSON::JSONBool(ref x) => write!(f, "{}", x),
            &JSON::JSONArray(ref x) => {
                write!(f, "[").unwrap();
                for (i, elem) in x.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ").unwrap();
                    }
                    write!(f, "{}", elem).unwrap();
                }
                write!(f, "]")
            }
            &JSON::JSONObject(ref x) => {
                write!(f, "{{").unwrap();
                for (i, (key, val)) in x.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ").unwrap();
                    }
                    write!(f, "\"{}\": {}", key, val).unwrap();
                }
                write!(f, "}}")
            }
        }
    }
}
