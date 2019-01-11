use std::fmt;
use std::error;

#[derive(Debug)]
pub enum HangeulError {
    NotASyllable,
}

impl fmt::Display for HangeulError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HangeulError::NotASyllable => write!(f, "HangeulError: Not a valid Hangeul syllable")
        }
    }
}

impl error::Error for HangeulError {
    fn description(&self) -> &str {
        match *self {
            HangeulError::NotASyllable => "HangeulError: Not a valid correct Hangeul syllable",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            _ => None
        }
    }
}
