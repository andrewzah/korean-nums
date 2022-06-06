use std::error;
use std::fmt;

#[derive(Debug)]
pub enum HangeulError {
    NotASyllable,
    Generic(String),
}

impl fmt::Display for HangeulError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HangeulError::NotASyllable => write!(f, "HangeulError: Not a valid Hangeul syllable"),
            HangeulError::Generic(msg) => write!(f, "HangeulError: {}", msg),
        }
    }
}

impl error::Error for HangeulError {
    fn description(&self) -> &str {
        match self {
            HangeulError::NotASyllable => "HangeulError: Not a valid correct Hangeul syllable",
            HangeulError::Generic(_) => "HangeulError: Generic",
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            _ => None,
        }
    }
}
