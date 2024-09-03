use core::fmt::Debug;
use std::fmt::Display;

use crate::models::core::TextSpan;

#[derive(Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: TextSpan,
}

impl Token {
    pub fn new(kind: TokenKind, span: TextSpan) -> Token {
        Token { kind, span }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.span.lexeme)
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.span.lexeme)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenKind {
    Identifier,
    Colon,
    Comma,
    Comment,
    Slash,
    EndOfFile,
    Equals,
    Invalid,
    LBrace,
    LBracket,
    LParen,
    Newline,
    RBrace,
    RBracket,
    RParen,
    Semicolon,
    Whitespace,
    Arrow,
    IntLiteral,
    FloatLiteral,
    StringLiteral,
    BoolLiteral,
    LessThan,
    LessThanEqual,
    Minus,
    GreaterThan,
    GreaterThanEqual,
    EqEq,
    NotEqual,
    Plus,
    Multiply,
    Mod,
}
