use crate::lexer::token::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ParseError {
    LexingError(LexingError),
    PrefixError(String),
    InfixError(String),
    InternalError(&'static str),

    ConsumeError {
        actual: Token,
        expected: String,
    },
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::LexingError(err) => write!(f, "{}", err.as_string()),
            Self::PrefixError(err) => write!(f, "{}", err),
            Self::InfixError(err) => write!(f, "{}", err),
            Self::InternalError(err) => write!(f, "An internal error has occured!\n\t{}", err),
            Self::ConsumeError { expected, actual } => write!(f, "expected '{}', but got '{}' instead", expected, actual),
        }
    }
}

impl From<LexingError> for ParseError {
    fn from(value: LexingError) -> Self {
        ParseError::LexingError(value)
    }
}

#[derive(Default, PartialEq, Eq, Clone, Hash)]
pub struct LexingError {
    cause: Option<String>,
}

impl LexingError {
    pub fn with_cause(cause: &'static str) -> Self {
        Self {
            cause: Some(cause.to_owned()),
        }
    }

    fn as_string(&self) -> String {
        let msg = "failed to lex token";

        if let Some(ref reason) = self.cause {
            format!("{}; because {}", msg, reason)
        } else {
            msg.to_string()
        }
    }
}

impl std::fmt::Debug for LexingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::fmt::Display for LexingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

impl std::error::Error for LexingError {}