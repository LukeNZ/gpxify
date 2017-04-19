use std::fmt;

pub enum GpxParserError {
    ExpectedLessThenToken,
    UnexpectedEOF
}

impl fmt::Display for GpxParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GpxParserError::ExpectedLessThenToken => write!(f, "Expected `<`, found "),
            GpxParserError::UnexpectedEOF => write!(f, "Unexpected end of file.")
        }
    }
}
