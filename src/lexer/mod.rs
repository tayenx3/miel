mod token;

use token::*;
use crate::common::{Operator, Span};
use codespan_reporting::diagnostic::{Diagnostic, Label};

pub fn tokenize(source_id: usize, source: &str, rodeo: &mut lasso::Rodeo) -> Result<Vec<Token>, Diagnostic<usize>> {
    let mut source_chars = source.char_indices().peekable();
    let mut tokens = Vec::new();

    while let Some((start, ch)) = source_chars.next() {
        match ch {
            ' ' | '\t' | '\n' | '\r' => continue,
            '+' => tokens.push(Token {
                kind: TokenKind::Operator(Operator::Plus),
                span: Span { start, end: start + ch.len_utf8(), source_id }
            }),
            '-' => tokens.push(Token {
                kind: TokenKind::Operator(Operator::Minus),
                span: Span { start, end: start + ch.len_utf8(), source_id }
            }),
            '*' => tokens.push(Token {
                kind: TokenKind::Operator(Operator::Star),
                span: Span { start, end: start + ch.len_utf8(), source_id }
            }),
            '/' => tokens.push(Token {
                kind: TokenKind::Operator(Operator::Slash),
                span: Span { start, end: start + ch.len_utf8(), source_id }
            }),
            '%' => tokens.push(Token {
                kind: TokenKind::Operator(Operator::Modulo),
                span: Span { start, end: start + ch.len_utf8(), source_id }
            }),
            '(' => tokens.push(Token {
                kind: TokenKind::LParen,
                span: Span { start, end: start + ch.len_utf8(), source_id }
            }),
            ')' => tokens.push(Token {
                kind: TokenKind::RParen,
                span: Span { start, end: start + ch.len_utf8(), source_id }
            }),
            '{' => tokens.push(Token {
                kind: TokenKind::LCurly,
                span: Span { start, end: start + ch.len_utf8(), source_id }
            }),
            '}' => tokens.push(Token {
                kind: TokenKind::RCurly,
                span: Span { start, end: start + ch.len_utf8(), source_id }
            }),
            ':' => if let Some(&(pos, ':')) = source_chars.peek() {
                source_chars.next();
                tokens.push(Token {
                    kind: TokenKind::CColon,
                    span: Span { start, end: pos + ':'.len_utf8(), source_id }
                });
            } else {
                tokens.push(Token {
                    kind: TokenKind::Colon,
                    span: Span { start, end: start + ch.len_utf8(), source_id }
                });
            },
            '0'..='9' => {
                let mut end: usize = start + 1;
                let mut is_float = false;
                while let Some((pos, ch)) = source_chars.next() {
                    if ch.is_ascii_digit() || ch == '_' {
                        end = pos + ch.len_utf8();
                    } else if ch == '.' && !is_float {
                        is_float = true;
                        end = pos + ch.len_utf8();
                    } else {
                        break;
                    }
                }
                let kind = if is_float {
                    TokenKind::FloatLit(source[start..end]
                        .replace('_', "")
                        .parse()
                        .unwrap())
                } else {
                    TokenKind::IntLit(source[start..end]
                        .replace('_', "")
                        .parse()
                        .unwrap())
                };
                tokens.push(Token {
                    kind,
                    span: Span {
                        start,
                        end: start + ch.len_utf8(),
                        source_id
                    }
                });
            },
            ch if ch.is_alphabetic() || ch == '_' => {
                let mut end: usize = start + 1;
                while let Some((pos, ch)) = source_chars.next() {
                    if ch.is_alphanumeric() || ch == '_' {
                        end = pos + ch.len_utf8();
                    } else {
                        break;
                    }
                }
                let kind = match &source[start..end] {
                    "proc" => TokenKind::KwProc,
                    "func" => TokenKind::KwFunc,
                    "callable" => TokenKind::KwCallable,
                    other => TokenKind::Identifier(rodeo.get_or_intern(other))
                };
                tokens.push(Token {
                    kind,
                    span: Span {
                        start,
                        end: start + ch.len_utf8(),
                        source_id
                    }
                });
            },
            other => return Err(
                Diagnostic::error()
                    .with_message(format!("unrecognized character `{other}`"))
                    .with_labels(vec![
                        Label::primary(source_id, start..(start+1))
                            .with_message("unknown character")
                    ])
            ),
        }
    }

    Ok(tokens)
}