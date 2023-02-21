#[derive(Debug)]
pub enum Error {
    NoMainFunctionError(NoMainFunctionError),
    MismatchedMainFunctionArgumentsError(MismatchedMainFunctionArgumentsError),
    LexError,
    ParseError,
    TypecheckError,
    IoError(std::io::Error),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::NoMainFunctionError(error) => write!(f, "{}", error),
            Error::MismatchedMainFunctionArgumentsError(error) => write!(f, "{}", error),
            Error::LexError => write!(f, "Error while lexing"),
            Error::ParseError => write!(f, "Error while parsing"),
            Error::TypecheckError => write!(f, "Error while typechecking"),
            Error::IoError(error) => write!(f, "{}", error),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IoError(error)
    }
}

pub struct NoMainFunctionError {}

impl std::fmt::Debug for NoMainFunctionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "No `main` function found")
    }
}

impl std::fmt::Display for NoMainFunctionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "No `main` function found")
    }
}

impl std::error::Error for NoMainFunctionError {}

pub struct MismatchedMainFunctionArgumentsError {
    signature_found: String,
}

impl MismatchedMainFunctionArgumentsError {
    pub fn new(signature_found: String) -> Self {
        Self { signature_found }
    }

    #[inline]
    pub fn signature_found(&self) -> String {
        self.signature_found.to_string()
    }
}

impl std::fmt::Debug for MismatchedMainFunctionArgumentsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Mismatched arguments for `main` function.\nCorrect signature should be `fn main(argc: i32, argv: [?]string) => i32 {{}}`\nGot signature `{}` instead.", self.signature_found)
    }
}

impl std::fmt::Display for MismatchedMainFunctionArgumentsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Mismatched arguments for `main` function.\nCorrect signature should be `fn main(argc: i32, argv: [?]string) => i32 {{}}`\nGot signature `{}` instead.", self.signature_found)
    }
}

impl std::error::Error for MismatchedMainFunctionArgumentsError {}
