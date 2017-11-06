use std::error::Error;
use std::fmt;
use std::io;
use std::str;

use subrecord::SubrecordContent;

#[derive(Debug)]
pub enum ParseError {
    Io(io::Error),
    Utf8(str::Utf8Error),
    UnknownFileType(i32),
    InvalidRecordName(String, String),
    InvalidRecordSize(u32, u32),
    InvalidSubrecordType(String, SubrecordContent)
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
            ParseError::InvalidRecordSize(_, _) => "invalid record size",
            ParseError::InvalidSubrecordType(_, _) => "invalid subrecords type"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ParseError::Io(ref err) => Some(err),
            ParseError::Utf8(ref err) => Some(err),
            ParseError::UnknownFileType(_) => None,
            ParseError::InvalidRecordName(_, _) => None,
            ParseError::InvalidRecordSize(_, _) => None,
            ParseError::InvalidSubrecordType(_, _) => None
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::Io(ref err) => write!(f, "{}: {}", self.description(), err),
            ParseError::Utf8(ref err) => write!(f, "{}: {}", self.description(), err),
            ParseError::UnknownFileType(num) => write!(f, "{}: {} indicatess no known file type", self.description(), num),
            ParseError::InvalidRecordName(ref expected, ref found) => write!(f, "{}: expected {}, found {}", self.description(), expected, found),
            ParseError::InvalidRecordSize(expected, found) => write!(f, "{}: expected {}, found {}", self.description(), expected, found),
            ParseError::InvalidSubrecordType(ref subrecord_name, ref found_type) => write!(f, "{}: subrecord {} cannot be of type {}", self.description(), subrecord_name, found_type.get_type_str())
        }
    }
}
