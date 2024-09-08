#[derive(Debug)]
pub enum Error {
    ParseError(&'static str),
    UnknownCharacterError(char),
}
