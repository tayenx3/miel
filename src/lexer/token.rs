use crate::common::{Operator, ReassignmentOp, Span};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    IntLit(lasso::Spur), FloatLit(lasso::Spur),
    BoolLit(bool), StringLit(lasso::Spur),
    Identifier(lasso::Spur),
    Operator(Operator),
    Reassign(ReassignmentOp),
    LParen, RParen, LCurly, RCurly,
    Colon, Comma,
    CColon, Walrus,
    Semicolon, Dot,
    KwCallable,
    KwNil,
    KwIf, KwThen, KwElse,
    KwWhile, KwDo,
    KwReturn,
    KwBreak, KwContinue,
}

impl TokenKind {
    pub fn format(&self, rodeo: &lasso::Rodeo) -> String {
        match self {
            Self::IntLit(s) | Self::FloatLit(s)
            | Self::Identifier(s) => rodeo.resolve(s).to_string(),
            Self::BoolLit(i) => if *i { "true" } else { "false" }.to_string(),
            Self::StringLit(i) => format!("\"{}\"", rodeo.resolve(i)),
            Self::Operator(o) => o.to_string(),
            Self::Reassign(o) => o.to_string(),
            Self::LParen => "(".to_string(),
            Self::RParen => ")".to_string(),
            Self::LCurly => "{".to_string(),
            Self::RCurly => "}".to_string(),
            Self::Colon => ":".to_string(),
            Self::Comma => ",".to_string(),
            Self::CColon => "::".to_string(),
            Self::Walrus => ":=".to_string(),
            Self::Semicolon => ";".to_string(),
            Self::Dot => ".".to_string(),
            Self::KwCallable => "callable".to_string(),
            Self::KwNil => "nil".to_string(),
            Self::KwIf => "if".to_string(),
            Self::KwThen => "then".to_string(),
            Self::KwElse => "else".to_string(),
            Self::KwWhile => "while".to_string(),
            Self::KwDo => "do".to_string(),
            Self::KwReturn => "return".to_string(),
            Self::KwBreak => "break".to_string(),
            Self::KwContinue => "continue".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span
}

impl Deref for Token {
    type Target = TokenKind;
    fn deref(&self) -> &Self::Target {
        &self.kind
    }
}

impl DerefMut for Token {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.kind
    }
}