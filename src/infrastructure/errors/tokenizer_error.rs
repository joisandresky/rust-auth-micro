use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use rusty_paseto::prelude::*;
use time::error::Format;

#[derive(Debug)]
pub enum TokenizerError {
    PasetoGenericParserError(GenericParserError),
    PasetoGenericBuilderError(GenericBuilderError),
    PasetoClaimerror(PasetoClaimError),
    TimeErrorFormat(Format)
}

impl fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TokenizerError::PasetoGenericParserError(msg) => write!(f, "Paseto Parser Error: {}", msg),
            TokenizerError::PasetoGenericBuilderError(msg) => write!(f, "Paseto Build Error: {}", msg),
            TokenizerError::PasetoClaimerror(msg) => write!(f, "Paseto Claim Error: {}", msg),
            TokenizerError::TimeErrorFormat(msg) => write!(f, "Time Format Error: {}", msg),
        }
    }
}

impl Error for TokenizerError{}

impl From<GenericParserError> for TokenizerError {
    fn from(value: GenericParserError) -> Self {
        TokenizerError::PasetoGenericParserError(value)
    }
}

impl From<GenericBuilderError> for TokenizerError {
    fn from(value: GenericBuilderError) -> Self {
        TokenizerError::PasetoGenericBuilderError(value)
    }
}

impl From<PasetoClaimError> for TokenizerError {
    fn from(value: PasetoClaimError) -> Self {
        TokenizerError::PasetoClaimerror(value)
    }
}

impl From<Format> for TokenizerError {
    fn from(value: Format) -> Self {
        TokenizerError::TimeErrorFormat(value)
    }
}