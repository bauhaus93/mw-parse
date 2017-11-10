use std::error::Error;
use std::fmt;
use std::io;
use std::str;

//use subrecord::field::Field;

#[derive(Debug)]
pub enum ParseError {
    Io(io::Error),
    Utf8(str::Utf8Error),
    UnknownFileType(i32),
    InvalidRecordName(String, String),
    InvalidSubrecordName(String, String),
    InvalidSubrecordSize(String, u32, u32)
}

impl From<io::Error> for ParseError {
    fn from(err: io::Error) -> ParseError {
        ParseError::Io(err)
    }
}

impl From<str::Utf8Error> for ParseError {
    fn from(err: str::Utf8Error) -> ParseError {
        ParseError::Utf8(err)
    }
}

impl Error for ParseError {

    fn description(&self) -> &str {
        match *self {
            ParseError::Io(_) => "io error",
            ParseError::Utf8(_) => "utf8 error",
            ParseError::UnknownFileType(_) => "unknown file type",
            ParseError::InvalidRecordName(_, _) => "invalid record name",
            ParseError::InvalidSubrecordName(_, _) => "invalid subrecord name",
            ParseError::InvalidSubrecordSize(_, _, _) => "invalid subrecord size"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ParseError::Io(ref err) => Some(err),
            ParseError::Utf8(ref err) => Some(err),
            ParseError::UnknownFileType(_) => None,
            ParseError::InvalidRecordName(_, _) => None,
            ParseError::InvalidSubrecordName(_, _) => None,
            ParseError::InvalidSubrecordSize(_, _, _) => None
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::Io(ref err) => write!(f, "{}: {}", self.description(), err),
            ParseError::Utf8(ref err) => write!(f, "{}: {}", self.description(), err),
            ParseError::UnknownFileType(num) => write!(f, "{}: {} indicatess no known file type", self.description(), num),
            ParseError::InvalidRecordName(ref expected, ref found) => write!(f, "{}: expected {}, but was {}", self.description(), expected, found),
            ParseError::InvalidSubrecordName(ref expected, ref found) => write!(f, "{}: expected {}, but was {}", self.description(), expected, found),
            ParseError::InvalidSubrecordSize(ref sub_name, size_expected, size_found) => write!(f, "{}: subrecord {}, expected size {}, but was {}", self.description(), sub_name, size_expected, size_found)
        }
    }
}
