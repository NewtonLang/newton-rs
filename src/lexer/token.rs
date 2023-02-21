/*
 * A representation of a token in Newton.
 * Newton (C) 2023
 */

use crate::types::types::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType<'a> {
    NullLiteral,
    Identifier(&'a str),
    DecLiteral(&'a str),
    FloatLiteral(&'a str),
    StringLiteral(&'a str),
    Char(&'a str),
    TypeIdentifier(Simple<'a>),

    Let,
    Fn,
    If,
    Else,
    Import,
    From,
    Return,
    Extern,
    While,
    Type,
    Struct,
    Trait,
    Implements,
    Enum,
    New,
    Delete,
    Sizeof,
    As,
    Static,
    Inline,
    Abstract,
    Mut,
    And,
    Or,
    For,
    Break,
    Continue,
    True,
    False,
    Match,
    Case,
    Default,
    Finally,
    Volatile,
    Register,

    Bang,
    Equals,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Smaller,
    Greater,
    Ampersand,
    Pipe,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Colon,
    Semicolon,
    Dot,
    Comma,
    Question,
    At,
    Caret,

    Varargs,
    EqualsEquals,
    BangEquals,
    SmallerEquals,
    GreaterEquals,
    AmpersandAmpersand,
    PipePipe,
    PlusPlus,
    MinusMinus,
    Arrow,
}

impl<'a> TokenType<'a> {
    pub fn precedence(&self) -> Precedence {
        match self {
            Self::Equals => Precedence::Assignment,
            Self::AmpersandAmpersand | Self::PipePipe => Precedence::And,
            Self::EqualsEquals | Self::BangEquals => Precedence::Equality,
            Self::Greater | Self::GreaterEquals | Self::Smaller | Self::SmallerEquals => {
                Precedence::Comparison
            }
            Self::Plus | Self::PlusPlus | Self::Minus | Self::MinusMinus => Precedence::Sum,
            Self::Star | Self::Slash | Self::Percent | Self::As => Precedence::Product,
            Self::LeftParen | Self::LeftBrace | Self::Dot => Precedence::Call,
            _ => Precedence::None,
        }
    }
}

impl<'a> std::fmt::Display for TokenType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::NullLiteral => write!(f, "null"),
            Self::Identifier(ref val) | Self::StringLiteral(ref val) | Self::Char(ref val) => {
                write!(f, "{val}")
            }
            Self::TypeIdentifier(val) => write!(f, "{val}"),
            Self::DecLiteral(val) | Self::FloatLiteral(val) => write!(f, "{val}"),

            Self::Let => write!(f, "let"),
            Self::Fn => write!(f, "fn"),
            Self::If => write!(f, "if"),
            Self::Else => write!(f, "else"),
            Self::Import => write!(f, "import"),
            Self::From => write!(f, "from"),
            Self::Return => write!(f, "return"),
            Self::Extern => write!(f, "extern"),
            Self::While => write!(f, "while"),
            Self::Type => write!(f, "type"),
            Self::Struct => write!(f, "struct"),
            Self::Trait => write!(f, "trait"),
            Self::Implements => write!(f, "implements"),
            Self::Enum => write!(f, "enum"),
            Self::New => write!(f, "new"),
            Self::Delete => write!(f, "delete"),
            Self::Sizeof => write!(f, "sizeof"),
            Self::As => write!(f, "as"),
            Self::Static => write!(f, "static"),
            Self::Inline => write!(f, "inline"),
            Self::Abstract => write!(f, "abstract"),
            Self::Mut => write!(f, "mut"),
            Self::And => write!(f, "and"),
            Self::Or => write!(f, "or"),
            Self::For => write!(f, "for"),
            Self::Break => write!(f, "break"),
            Self::Continue => write!(f, "continue"),
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::Match => write!(f, "match"),
            Self::Case => write!(f, "case"),
            Self::Default => write!(f, "default"),
            Self::Finally => write!(f, "finally"),
            Self::Volatile => write!(f, "volatile"),
            Self::Register => write!(f, "register"),

            Self::Bang => write!(f, "!"),
            Self::Equals => write!(f, "="),
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Star => write!(f, "*"),
            Self::Slash => write!(f, "/"),
            Self::Percent => write!(f, "%"),
            Self::Smaller => write!(f, "<"),
            Self::Greater => write!(f, ">"),
            Self::Ampersand => write!(f, "&"),
            Self::Pipe => write!(f, "|"),
            Self::LeftParen => write!(f, "("),
            Self::RightParen => write!(f, ")"),
            Self::LeftBrace => write!(f, "{{"),
            Self::RightBrace => write!(f, "}}"),
            Self::LeftBracket => write!(f, "["),
            Self::RightBracket => write!(f, "]"),
            Self::Colon => write!(f, ":"),
            Self::Semicolon => write!(f, ";"),
            Self::Dot => write!(f, "."),
            Self::Comma => write!(f, ","),
            Self::Question => write!(f, "?"),
            Self::At => write!(f, "@"),
            Self::Caret => write!(f, "^"),

            Self::Varargs => write!(f, "..."),
            Self::EqualsEquals => write!(f, "=="),
            Self::BangEquals => write!(f, "!="),
            Self::SmallerEquals => write!(f, "<="),
            Self::GreaterEquals => write!(f, ">="),
            Self::AmpersandAmpersand => write!(f, "&&"),
            Self::PipePipe => write!(f, "||"),
            Self::PlusPlus => write!(f, "++"),
            Self::MinusMinus => write!(f, "--"),
            Self::Arrow => write!(f, "=>"),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    None = 0,
    Assignment = 1,
    And = 2,
    Equality = 3,
    Comparison = 4,
    Sum = 5,
    Product = 6,
    Unary = 7,
    Call = 8,
}
