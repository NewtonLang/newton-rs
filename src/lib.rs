pub mod ir;
pub mod ast;
pub mod lexer;
pub mod types;
pub mod parser;
pub mod codegen;
pub mod semantic;

use parser::span::Span;

#[derive(Debug)]
pub struct Source {
    name: String,
    code: String,
}

impl Source {
    pub fn new(name: &str, code: &str) -> Self {
        Self {
            name: name.to_owned(),
            code: code.to_owned(),
        }
    }

    pub fn slice(&self, span: Span) -> &str {
        &self.code[span.start() ..= span.end()]
    }

    #[inline]
    pub fn name(&self) -> &String {
        &self.name
    }

    #[inline]
    pub fn code(&self) -> &String {
        &self.code
    }
}

impl PartialEq for Source {
    fn eq(&self, other: &Source) -> bool {
        self.name == other.name
    }
}

impl Eq for Source {}

impl std::borrow::Borrow<str> for &Source {
    fn borrow(&self) -> &str {
        self.name.as_str()
    }
}

impl std::hash::Hash for Source {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}