use crate::token::Token;

#[derive(Debug)]
pub enum Error {
    ParseError(String),
    UnknownCharacterError(char),
}

impl Error {
    pub fn from_str(err: &str) -> Error {
        Error::ParseError(format!("{}", err))
    }

    pub fn from_token(err: &str, token: &Token) -> Error {
        Error::ParseError(format!("{}: {:?}", err, token))
    }
}
