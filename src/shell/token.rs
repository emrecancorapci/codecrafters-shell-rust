use std::fmt::Display;

#[derive(PartialEq, Eq, Debug)]
pub enum Token {
    Space,
    Value(String),
    Argument(String, bool),
    String(String, bool),
}

impl Token {
    pub fn get_value(&self) -> String {
        match self{
            Token::Space => String::from(" "),
            Token::Value(val) => val.to_string(),
            Token::Argument(val, _) => val.to_string(),
            Token::String(val, _) => val.to_string(),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}