use crate::Source;
use super::token::*;
use crate::parser::span::*;
use crate::types::types::*;
use crate::parser::error::*;

macro_rules! consume_once {
    ($self: ident, $start: ident, $token: expr) => {{
        $self.advance();
        Ok($self.spanned($start, $token))
    }};
}

macro_rules! consume_multiple {
    ($self: ident, $start: ident, $first: expr, $second: expr) => {{
        let InputPosition { value: tok, .. } = $self.current.unwrap();
        $self.advance();

        if let Some(InputPosition { value: new, .. }) = $self.current {
            if new == tok {
                $self.advance();
                Ok($self.spanned($start, $second))
            } else {
                Ok($self.spanned($start, $first))
            }
        } else {
            Ok($self.spanned($start, $first))
        }
    }};

    ($self: ident, $start: ident, $next: expr, $first: expr, $second: expr) => {{
        $self.advance();

        if let Some(InputPosition { value: new, .. }) = $self.current {
            if new == $next {
                $self.advance();
                Ok($self.spanned($start, $second))
            } else {
                Ok($self.spanned($start, $first))
            }
        } else {
            Ok($self.spanned($start, $first))
        }
    }};
} 

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct InputPosition {
    pos: usize,
    value: char,
}

impl InputPosition {
    fn new_opt(value: Option<(usize, char)>) -> Option<Self> {
        let (pos, value) = value?;

        Some(Self {
            pos,
            value,
        })
    }
}

pub type Scanned<'a> = Result<Spanned<TokenType<'a>>, Spanned<ParseError<'a>>>;

pub trait Scanner<'a>: Iterator<Item = Scanned<'a>> {
    fn source(&self) -> &'a Source;
}

pub struct Lexer<'a> {
    source: &'a Source,
    src: &'a str,
    chars: std::iter::Peekable<std::str::CharIndices<'a>>,
    current: Option<InputPosition>,
    prev: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a Source) -> Self {
        let src = &source.code;
        let mut chars = src.char_indices().peekable();

        Self {
            source,
            src,
            current: InputPosition::new_opt(chars.next()),
            chars,
            prev: None,
        }
    }

    fn pos(&self) -> usize {
        if let Some(InputPosition { pos, .. }) = self.current {
            return pos;
        }

        self.src.len()
    }

    fn slice(&self, start: usize, end: usize) -> &'a str {
        let end = if end > self.src.len() {
            self.src.len()
        } else {
            end
        };

        &self.src[ start .. end ]
    }

    fn spanned<T>(&self, start: usize, t: T) -> Spanned<T> {
        Spanned::new(start, self.pos() - self.prev.map_or(0, char::len_utf8), t)
    }

    fn advance(&mut self) -> Option<InputPosition> {
        let current = self.current?;

        if self.pos() > 0 {
            self.prev = Some({
                let InputPosition { value: prev, .. } = current;
                prev
            });
        }

        self.current = InputPosition::new_opt(self.chars.next());
        Some(current)
    }

    fn read_while<T>(&mut self, callback: T) -> &'a str where T: Fn(char) -> bool {
        let start = self.pos();

        while let Some(InputPosition { value, .. }) = self.current {
            if callback(value) {
                self.advance();
            } else {
                break;
            }
        }

        self.slice(start, self.pos())
    }

    fn skip_whitespace(&mut self) {
        self.read_while(char::is_whitespace);
    }

    fn scan_identifier(&mut self) -> Scanned<'a> {
        let start = self.pos();
        let slice = self.read_while(| c | c.is_alphanumeric() || c == '_');

        if let Some(keyword) = self.check_keyword(start, slice) {
            return Ok(keyword);
        }

        if slice.is_ascii() {
            Ok(self.spanned(start, TokenType::Identifier(slice)))
        } else {
            Err(self.spanned(start, ParseError::LexingError(LexingError::with_cause("non-ascii identifiers are not allowed"))))
        }
    }

    fn check_keyword(&mut self, start: usize, slice: &'a str) -> Option<Spanned<TokenType<'a>>> {
        Some(self.spanned(start, match slice {
            // base types
            "string" => TokenType::TypeIdentifier(Simple::String),
            "i8" => TokenType::TypeIdentifier(Simple::Integer(Integer::new_signed_int(8))),
            "u8" => TokenType::TypeIdentifier(Simple::Integer(Integer::new_unsigned_int(8))),
            "i16" => TokenType::TypeIdentifier(Simple::Integer(Integer::new_signed_int(16))),
            "u16" => TokenType::TypeIdentifier(Simple::Integer(Integer::new_unsigned_int(16))),
            "i32" => TokenType::TypeIdentifier(Simple::Integer(Integer::new_signed_int(32))),
            "u32" => TokenType::TypeIdentifier(Simple::Integer(Integer::new_unsigned_int(32))),
            "i64" => TokenType::TypeIdentifier(Simple::Integer(Integer::new_signed_int(64))),
            "u64" => TokenType::TypeIdentifier(Simple::Integer(Integer::new_unsigned_int(64))),
            "f32" => TokenType::TypeIdentifier(Simple::Float(Float::new_f32())),
            "f64" => TokenType::TypeIdentifier(Simple::Float(Float::new_f64())),
            "char" => TokenType::TypeIdentifier(Simple::Character),
            "void" => TokenType::TypeIdentifier(Simple::Void),
            "bool" => TokenType::TypeIdentifier(Simple::Bool),

            "let" => TokenType::Let,
            "fn" => TokenType::Fn,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "import" => TokenType::Import,
            "from" => TokenType::From,
            "return" => TokenType::Return,
            "extern" => TokenType::Extern,
            "while" => TokenType::While,
            "type" => TokenType::Type,
            "struct" => TokenType::Struct,
            "trait" => TokenType::Trait,
            "implements" => TokenType::Implements,
            "enum" => TokenType::Enum,
            "new" => TokenType::New,
            "delete" => TokenType::Delete,
            "sizeof" => TokenType::Sizeof,
            "as" => TokenType::As,
            "static" => TokenType::Static,
            "inline" => TokenType::Inline,
            "abstract" => TokenType::Abstract,
            "mut" => TokenType::Mut,
            "and" => TokenType::And,
            "or" => TokenType::Or,
            "for" => TokenType::For,
            "break" => TokenType::Break,
            "continue" => TokenType::Continue,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "match" => TokenType::Match,
            "case" => TokenType::Case,
            "default" => TokenType::Default,
            "finally" => TokenType::Finally,
            "volatile" => TokenType::Volatile,
            "register" => TokenType::Register,

            _ => return None,
        }))
    }

    fn scan_number(&mut self) -> Scanned<'a> {
        let start = self.pos();
        let slice = self.read_while(| c | c.is_digit(10));

        if let Some(InputPosition { value: '.', .. }) = self.current {
            if let Some((_, peek)) = self.chars.peek() {
                if peek.is_digit(10) {
                    self.advance();
                    self.read_while(| c | c.is_digit(10));

                    let slice = self.slice(start, self.pos());
                    return Ok(self.spanned(start, TokenType::FloatLiteral(slice)));
                }
            }
        }

        Ok(self.spanned(start, TokenType::DecLiteral(slice)))
    }

    fn scan_string(&mut self) -> Scanned<'a> {
        self.advance();

        let start = self.pos();
        let slice = self.read_while(| c | c != '"');

        if self.advance().is_none() {
            let pos = self.pos();

            Err(Spanned::new(pos, pos, ParseError::LexingError(LexingError::with_cause("unterminated string literal"))))?;
        }

        let mut spanned = self.spanned(start, TokenType::StringLiteral(slice));

        if spanned.span.end - spanned.span.start > 0 {
            spanned.span.end -= 1;
        }

        Ok(spanned)
    }

    fn scan_token(&mut self) -> Option<Scanned<'a>> {
        self.skip_whitespace();

        let start = self.pos();
        let ch = self.current.map(| InputPosition { value, .. } | value)?;

        let scanned: Scanned = match ch {
            '=' => {
                let token = match self.chars.peek()? {
                    (_, '=') => TokenType::EqualsEquals,
                    (_, '>') => TokenType::Arrow,

                    (_, _) => TokenType::Equals,
                };

                self.advance();
                self.advance();

                Ok(self.spanned(start, token))
            },

            '/' => {
                if let Some((_, '/')) = self.chars.peek() {
                    self.read_while(| c | c != '\n');
                    return self.scan_token();
                }

                consume_once!(self, start, TokenType::Slash)
            },

            '.' => {
                let dots = self.read_while(| c | c == '.');

                Ok(self.spanned(start, match dots.len() {
                    1 => TokenType::Dot,
                    3 => TokenType::Varargs,

                    _ => {
                        return Some(Err(Spanned::new(start, self.pos() - 1, ParseError::LexingError(LexingError::with_cause("too many dots")))));
                    }
                }))
            },

            '\'' => {
                self.advance();

                let c = self.read_while(| c | c != '\'');
                let result = match c.len() {
                    1 if c != "\\" => Ok(Spanned::new(start + 1, start + 1, TokenType::Char(&c[ .. ]))),
                    2 if c == "\\\\" => Ok(Spanned::new(start + 1, start + 1, TokenType::Char("\\"))),
                    2 if c == "\\0" => Ok(Spanned::new(start + 1, start + 1, TokenType::Char("\0"))),
                    2 if c == "\\n" => Ok(Spanned::new(start + 1, start + 1, TokenType::Char("\n"))),
                    2 if c == "\\r" => Ok(Spanned::new(start + 1, start + 1, TokenType::Char("\r"))),
                    2 if c == "\\t" => Ok(Spanned::new(start + 1, start + 1, TokenType::Char("\t"))),

                    _ => Err(Spanned::new(start, self.pos(), ParseError::LexingError(LexingError::with_cause("`char` must have a length of one"))))
                };

                self.advance();

                result
            },

            '!' => consume_multiple!(self, start, '=', TokenType::Bang, TokenType::BangEquals),
            '+' => consume_multiple!(self, start, TokenType::Plus, TokenType::PlusPlus),
            '-' => consume_multiple!(self, start, TokenType::Minus, TokenType::MinusMinus),
            '<' => consume_multiple!(self, start, '=', TokenType::Smaller, TokenType::SmallerEquals),
            '>' => consume_multiple!(self, start, '=', TokenType::Greater, TokenType::GreaterEquals),
            '&' => consume_multiple!(self, start, TokenType::Ampersand, TokenType::AmpersandAmpersand),
            '|' => consume_multiple!(self, start, TokenType::Pipe, TokenType::PipePipe),
            '*' => consume_once!(self, start, TokenType::Star),
            '%' => consume_once!(self, start, TokenType::Percent),
            ':' => consume_once!(self, start, TokenType::Colon),
            ';' => consume_once!(self, start, TokenType::Semicolon),
            '(' => consume_once!(self, start, TokenType::LeftParen),
            ')' => consume_once!(self, start, TokenType::RightParen),
            '{' => consume_once!(self, start, TokenType::LeftBrace),
            '}' => consume_once!(self, start, TokenType::RightBrace),
            '[' => consume_once!(self, start, TokenType::LeftBracket),
            ']' => consume_once!(self, start, TokenType::RightBracket),
            '?' => consume_once!(self, start, TokenType::Question),
            '@' => consume_once!(self, start, TokenType::At),
            '^' => consume_once!(self, start, TokenType::Caret),
            ',' => consume_once!(self, start, TokenType::Comma),

            '"' => self.scan_string(),
            c if c.is_alphabetic() => self.scan_identifier(),
            c if c.is_digit(10) => self.scan_number(),

            _ => {
                self.advance();
                
                let span = Span::new(start, start);
                Err(Spanned { span, node: ParseError::LexingError(LexingError::default()) })
            }
        };

        Some(scanned)
    }

    
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Scanned<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.scan_token()
    }
}

impl<'a> Scanner<'a> for Lexer<'a> {
    fn source(&self) -> &'a Source {
        &self.source
    }
}