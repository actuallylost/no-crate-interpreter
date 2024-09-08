use crate::error::Error;

pub struct Parser;

impl Parser {
    pub fn parse(input: &str) -> Result<(), Error> {
        if input == "a" {
            return Ok(());
        }

        if input.len() == 1 {
            return Err(Error::ParseError("input is not parsable"));
        }

        let (left, right) = (&input[..1], &input[1..]);
        Parser::parse(left)?;
        Parser::parse(right)?;

        Ok(())
    }
}
