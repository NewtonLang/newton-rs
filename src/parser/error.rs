use crate::lexer::token::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ParseError<'a> {
    LexingError(LexingError<'a>),
    PrefixError(&'a str),
    InfixError(&'a str),
    InternalError(&'a str),

    ConsumeError {
        actual: TokenType<'a>,
        expected: &'a str,
    },
}

impl<'a> std::fmt::Display for ParseError<'a> {
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

impl<'a> From<LexingError<'a>> for ParseError<'a> {
    fn from(value: LexingError<'a>) -> Self {
        ParseError::LexingError(value)
    }
}

#[derive(Default, PartialEq, Eq, Clone, Hash)]
pub struct LexingError<'a> {
    cause: Option<&'a str>,
}

impl<'a> LexingError<'a> {
    pub fn with_cause(cause: &'static str) -> Self {
        Self {
            cause: Some(cause),
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

impl<'a> std::fmt::Debug for LexingError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl<'a> std::fmt::Display for LexingError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

impl<'a> std::error::Error for LexingError<'a> {}