use crate::common::{flag::Flag, text::Span};

pub struct Token {
    kind: TokenKind,
    span: Span,
    flag: Flag,
}

pub enum TokenKind {
    // special

    Null,

    // literals
    
    Identifier(String),
    String(String),
    Integer(i32),
    Float(f32),
    Boolean(bool),

    // punctuation

    Bang,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Arrow,
    Comma,
    Semicolon,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span, flag: Flag) -> Self {
        Self { kind, span, flag }
    }

    pub fn span(&self) -> &Span {
        &self.span
    }

    pub fn flag(&self) -> &Flag {
        &self.flag
    }

    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }
}

impl std::ops::Deref for Token {
    type Target = TokenKind;

    fn deref(&self) -> &Self::Target {
        &self.kind
    }
}
