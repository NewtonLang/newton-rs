pub mod ir;
pub mod ast;
pub mod lexer;
pub mod types;
pub mod parser;
pub mod codegen;
pub mod semantic;

use parser::span::Span;

#[derive(Debug)]
pub struct Source<'a> {
    name: &'a str,
    code: &'a str,
}

impl<'a> Source<'a> {
    pub fn new(name: &'a str, code: &'a str) -> Self {
        Self {
            name,
            code,
        }
    }

    pub fn slice(&self, span: Span) -> &str {
        &self.code[ span.start ..= span.end ]
    }

    #[inline]
    pub fn name(&self) -> &'a str {
        self.name
    }

    #[inline]
    pub fn code(&self) -> &'a str {
        self.code
    }
}

impl<'a> PartialEq for Source<'a> {
    fn eq(&self, other: &Source) -> bool {
        self.name == other.name
    }
}

impl<'a> Eq for Source<'a> {}

impl<'a> std::borrow::Borrow<str> for &Source<'a> {
    fn borrow(&self) -> &str {
        self.name
    }
}

impl<'a> std::hash::Hash for Source<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}