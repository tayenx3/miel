use crate::common::{Operator, Span};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    IntLit(i64), FloatLit(f64),
    Identifier(lasso::Spur),
    Operator(Operator),
    LParen, RParen, LCurly, RCurly,
    Colon, CColon,
    KwProc, KwFunc, KwCallable
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span
}