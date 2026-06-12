use crate::common::{Operator, Span};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind<'tok> {
    IntLit(i64), FloatLit(f64),
    StringLit(&'tok str),
    Identifier(lasso::Spur),
    Operator(Operator),
    LParen, RParen, LCurly, RCurly,
    Colon, Comma,
    CColon, Walrus, Assign,
    Semicolon,
    KwCallable,
    KwNil,
}

impl<'tok> TokenKind<'tok> {
    pub fn format(&self, rodeo: &lasso::Rodeo) -> String {
        match self {
            Self::IntLit(i) => i.to_string(),
            Self::FloatLit(i) => i.to_string(),
            Self::StringLit(i) => format!("\"{i}\""),
            Self::Identifier(s) => rodeo.resolve(s).to_string(),
            Self::Operator(o) => o.to_string(),
            Self::LParen => "(".to_string(),
            Self::RParen => ")".to_string(),
            Self::LCurly => "{".to_string(),
            Self::RCurly => "}".to_string(),
            Self::Colon => ":".to_string(),
            Self::Comma => ",".to_string(),
            Self::CColon => "::".to_string(),
            Self::Walrus => ":=".to_string(),
            Self::Assign => "=".to_string(),
            Self::Semicolon => ';'.to_string(),
            Self::KwCallable => "callable".to_string(),
            Self::KwNil => "nil".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Token<'tok> {
    pub kind: TokenKind<'tok>,
    pub span: Span
}

impl<'tok> Deref for Token<'tok> {
    type Target = TokenKind<'tok>;
    fn deref(&self) -> &Self::Target {
        &self.kind
    }
}

impl<'tok> DerefMut for Token<'tok> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.kind
    }
}