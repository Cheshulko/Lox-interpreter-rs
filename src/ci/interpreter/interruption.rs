use std::{error::Error, fmt::Display};

use super::Evaluation;

#[derive(Debug)]
pub enum Interruption<'de> {
    Error(anyhow::Error),
    Return(Evaluation<'de>),
}

impl Display for Interruption<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Interruption::Error(err) => write!(f, "Error: {}", err),
            Interruption::Return(_) => write!(f, "Return"),
        }
    }
}

impl Error for Interruption<'_> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Interruption::Error(err) => err.source(),
            Interruption::Return(_) => None,
        }
    }
}

impl<'de> From<anyhow::Error> for Interruption<'de> {
    fn from(err: anyhow::Error) -> Interruption<'de> {
        Interruption::Error(err)
    }
}
