/*
 * A representation of a token in Newton.
 * Newton (C) 2023
 */

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token {
    lexeme: &'static str,
    token_type: TokenType,
    location: (u8, u8)
}

impl Token {
    pub fn new(&self, lexeme: &'static str, token_type: TokenType, location: (u8, u8)) -> Self {
        Self {
            lexeme,
            token_type,
            location,
        }
    }

    pub fn from(&self, lexeme: &'static str, token_type: TokenType) -> Self {
        Self {
            lexeme,
            token_type,
            location: (0, 0),
        }
    }

    #[inline]
    pub fn lexeme(&mut self) -> String {
        self.lexeme.to_owned()
    }

    #[inline]
    pub fn token_type(&mut self) -> TokenType {
        self.token_type
    }

    #[inline]
    pub fn location(&mut self) -> (u8, u8) {
        self.location
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    Eof = -1,
    Newline = 0,
    Integer = 1,
    Float = 2,
    Identifier = 3,
    String = 4,
    Character = 5,

    Plus = 6,
    Minus = 7,
    Star = 8,
    Slash = 9,
    Percent = 10,
    Ampersand = 11,
    Pipe = 12,
    Caret = 13,
    Bang = 14,
    Question = 15,
    Eq = 16,
    EqEq = 17,
    NEq = 18,
    Gt = 19,
    GtEq = 20,
    Lt = 21,
    LtEq = 22,
    PlusPlus = 23,
    MinusMinus = 24,
    PlusEq = 25,
    MinusEq = 26,
    StarEq = 27,
    SlashEq = 28,
    PercentEq = 29,
    AmpersandEq = 30,
    PipeEq = 31,
    CaretEq = 32,
    Arrow = 33,
    Dot = 34,
    Colon = 35,
    Semicolon = 36,
    Comma = 37,
    LParen = 38,
    RParen = 39,
    LBrace = 40,
    RBrace = 41,
    LBracket = 42,
    RBracket = 43,
    GtGt = 44,
    LtLt = 45,
    GtGtEq = 46,
    LtLtEq = 47,

    Let = 101,
    Fn = 102,
    If = 103,
    Else = 104,
    Import = 105,
    From = 106,
    Return = 107,
    Extern = 108,
    While = 109,
    Type = 110,
    Struct = 111,
    Trait = 112,
    Implements = 113,
    Enum = 114,
    New = 115,
    Delete = 116,
    Sizeof = 117,
    As = 118,
    Static = 119,
    Inline = 120,
    Abstract = 121,
    Mut = 122,
    And = 123,
    Or = 124,
    For = 125,
    Break = 126,
    Continue = 127,
    True = 128,
    False = 129,
    Null = 130,
    Match = 131,
    Case = 132,
    Default = 133,
    Finally = 134,
    Volatile = 135,
    Register = 136
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Eof => write!(f, "<eof>"),
            Self::Newline => write!(f, "<newline>"),
            Self::Integer => write!(f, "<integer>"),
            Self::Float => write!(f, "<float>"),
            Self::Identifier => write!(f, "<identifier>"),
            Self::String => write!(f, "<string>"),
            Self::Character => write!(f, "<character>"),

            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Star => write!(f, "*"),
            Self::Slash => write!(f, "/"),
            Self::Percent => write!(f, "%"),
            Self::Ampersand => write!(f, "&"),
            Self::Pipe => write!(f, "|"),
            Self::Caret => write!(f, "^"),
            Self::Bang => write!(f, "!"),
            Self::Question => write!(f, "?"),
            Self::Eq => write!(f, "="),

            // TODO : finish adding all the token string representations

            _ => write!(f, "INTERNAL ERROR: Unknown token! If you believe that this is a mistake, please open a new issue on the repository!")
        }
    }
}