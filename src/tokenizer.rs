use std::char;

pub struct Token {
    typ: String,
    value: String,
}

pub fn token(c: &str) -> Token {
    let mut tkn = Token {
        typ: "".to_string(),
        value: "".to_string(),
    };
    if c == "+" {
        tkn = Token {
            typ: "Plus".to_string(),
            value: c.to_string(),
        }
    } else {
        tkn = Token {
            typ: "undefined".to_string(),
            value: c.to_string(),
        }
    }

    return tkn;
}
