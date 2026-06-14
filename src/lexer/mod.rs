pub mod token;

use token::*;
use crate::common::{Operator, Span, Diag, Label};

fn skip_block_comment(
    source_id: usize,
    source_chars: &mut std::iter::Peekable<std::str::CharIndices>,
    start: usize
) -> Result<usize, Diag> {
    let mut end = start + 1;
    while let Some((pos, ch)) = source_chars.next() {
        end = pos + ch.len_utf8();
        if ch == ';' && let Some((_, '[')) = source_chars.peek() {
            source_chars.next();
            end = skip_block_comment(source_id, source_chars, pos)? + ']'.len_utf8();
        }
        if ch == ']' {
            return Ok(end);
        }
    }
    Err(Diag::error()
        .with_message(format!("Unterminated block comment"))
        .with_labels(vec![
            Label::primary(source_id, start..end)
                .with_message("unterminated block comment")
        ]))
}

pub fn tokenize<'lex>(source_id: usize, source: &'lex str, rodeo: &mut lasso::Rodeo) -> Result<Vec<Token<'lex>>, Diag> {
    let mut source_chars = source.char_indices().peekable();
    let mut tokens = Vec::new();

    'outer: while let Some((start, ch)) = source_chars.next() {
        match ch {
            ' ' | '\t' | '\n' | '\r' => continue,
            ';' => if let Some((_, ';')) = source_chars.peek() {
                while let Some((_, ch)) = source_chars.next() {
                    if ch == '\n' { break }
                }
            } else if let Some((_, '[')) = source_chars.peek() {
                source_chars.next();
                skip_block_comment(source_id, &mut source_chars, start)?;
            } else {
                tokens.push(Token {
                    kind: TokenKind::Semicolon,
                    span: Span { start, end: start + ch.len_utf8(), source_id }
                });
            },
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
            } else if let Some(&(pos, '=')) = source_chars.peek() {
                source_chars.next();
                tokens.push(Token {
                    kind: TokenKind::Walrus,
                    span: Span { start, end: pos + '='.len_utf8(), source_id }
                });
            } else {
                tokens.push(Token {
                    kind: TokenKind::Colon,
                    span: Span { start, end: start + ch.len_utf8(), source_id }
                });
            },
            ',' => tokens.push(Token {
                kind: TokenKind::Comma,
                span: Span { start, end: start + ch.len_utf8(), source_id }
            }),
            '=' => tokens.push(Token {
                kind: TokenKind::Assign,
                span: Span { start, end: start + ch.len_utf8(), source_id }
            }),
            '0'..='9' => {
                let mut end: usize = start + 1;
                let mut is_float = false;
                while let Some(&(pos, ch)) = source_chars.peek() {
                    if ch.is_ascii_digit() || ch == '_' {
                        end = pos + ch.len_utf8();
                    } else if ch == '.' && !is_float {
                        is_float = true;
                        end = pos + ch.len_utf8();
                    } else {
                        end = pos;
                        break;
                    }
                    source_chars.next();
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
                    span: Span { start, end, source_id }
                });
            },
            ch if ch.is_alphabetic() || ch == '_' => {
                let mut end: usize = start + 1;
                while let Some(&(pos, ch)) = source_chars.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        end = pos + ch.len_utf8();
                    } else {
                        end = pos;
                        break;
                    }
                    source_chars.next();
                }
                let kind = match &source[start..end] {
                    "callable" => TokenKind::KwCallable,
                    "nil" => TokenKind::KwNil,
                    other => TokenKind::Identifier(rodeo.get_or_intern(other))
                };
                tokens.push(Token {
                    kind,
                    span: Span { start, end, source_id }
                });
            },
            '"' => {
                let mut end: usize = start + 1;
                while let Some((pos, ch)) = source_chars.next() {
                    end = pos + ch.len_utf8();
                    if ch == '"' {
                        tokens.push(Token {
                            kind: TokenKind::StringLit(&source[(start + 1)..pos]),
                            span: Span { start, end, source_id }
                        });
                        continue 'outer;
                    }
                }
                return Err(Diag::error()
                    .with_message(format!("Unterminated string"))
                    .with_labels(vec![
                        Label::primary(source_id, start..end)
                            .with_message("unterminated string")
                    ]));
            },
            other => return Err(Diag::error()
                .with_message(format!("Unrecognized character `{other}`"))
                .with_labels(vec![
                    Label::primary(source_id, start..(start+1))
                        .with_message("unknown character")
                ])),
        }
    }

    Ok(tokens)
}
