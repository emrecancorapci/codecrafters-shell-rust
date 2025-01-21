use std::fmt::Display;

#[derive(PartialEq, Eq, Debug)]
pub enum Token {
    Space,
    Value(String),
    Argument(String, bool),
    String(String, bool),
    Redirector(u8),
    Appender(u8),
}

impl Token {
    pub fn get_value(&self) -> String {
        match self{
            Token::Space => String::from(" "),
            Token::Value(val) => val.to_string(),
            Token::Argument(val, _) => val.to_string(),
            Token::String(val, _) => val.to_string(),
            Token::Redirector(_) => todo!(),
            Token::Appender(_) => todo!(),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Clone for Token {
    fn clone(&self) -> Self {
        match self {
            Self::Space => Self::Space,
            Self::Value(arg0) => Self::Value(arg0.clone()),
            Self::Argument(arg0, arg1) => Self::Argument(arg0.clone(), arg1.clone()),
            Self::String(arg0, arg1) => Self::String(arg0.clone(), arg1.clone()),
            Self::Redirector(arg0) => Self::Redirector(arg0.clone()),
            Self::Appender(arg0) => Self::Appender(arg0.clone()),
        }
    }
}