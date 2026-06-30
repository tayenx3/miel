//! Lexer - Lexical Analysis

// todo: ADD TESTS!!!

pub mod token;

use token::*;
use crate::common::{Diag, Label, Operator, ReassignmentOp, Span};

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
            end = skip_block_comment(source_id, source_chars, pos)?;
        }
        if ch == ']' {
            return Ok(end);
        }
    }
    Err(Diag::error()
        .with_message("Unterminated block comment")
        .with_labels(vec![
            Label::primary(source_id, start..end)
                .with_message("unterminated block comment")
        ]))
}

// todo: add binary, hex and octal literal parsing
fn lex_num<'lex>(
    rodeo: &mut lasso::Rodeo,
    source_id: usize,
    source_chars: &mut std::iter::Peekable<std::str::CharIndices<'lex>>,
    source: &'lex str,
    start: usize,
) -> Token {
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
        TokenKind::FloatLit(rodeo.get_or_intern(&source[start..end]))
    } else {
        TokenKind::IntLit(rodeo.get_or_intern(&source[start..end]))
    };
    Token { kind, span: Span { start, end, source_id } }
}

fn lex_ident<'lex>(
    rodeo: &mut lasso::Rodeo,
    source_id: usize,
    source_chars: &mut std::iter::Peekable<std::str::CharIndices<'lex>>,
    source: &'lex str,
    start: usize,
) -> Token {
    let mut end = start + 1;
    while let Some((pos, ch)) = source_chars.next_if(|(_, ch)| ch.is_alphanumeric() || *ch == '_') {
        end = pos + ch.len_utf8();
    }
    let kind = match &source[start..end] {
        "true" => TokenKind::BoolLit(true),
        "false" => TokenKind::BoolLit(false),
        "callable" => TokenKind::KwCallable,
        "nil" => TokenKind::KwNil,
        "if" => TokenKind::KwIf,
        "then" => TokenKind::KwThen,
        "else" => TokenKind::KwElse,
        "while" => TokenKind::KwWhile,
        "do" => TokenKind::KwDo,
        "return" => TokenKind::KwReturn,
        "break" => TokenKind::KwBreak,
        "continue" => TokenKind::KwContinue,
        "or" => TokenKind::Operator(Operator::KwOr),
        "and" => TokenKind::Operator(Operator::KwAnd),
        "xor" => TokenKind::Operator(Operator::KwXor),
        "not" => TokenKind::Operator(Operator::KwNot),
        other => TokenKind::Identifier(rodeo.get_or_intern(other))
    };
    Token { kind, span: Span { start, end, source_id } }
}

fn lex_str<'lex>(
    rodeo: &mut lasso::Rodeo,
    source_id: usize,
    source_chars: &mut std::iter::Peekable<std::str::CharIndices<'lex>>,
    source: &'lex str,
    start: usize,
) -> Result<Token, Diag> {
    let mut end = start + 1;
    for (pos, ch) in source_chars.by_ref() {
        end = pos + ch.len_utf8();
        if ch == '"' {
            return Ok(Token {
                kind: TokenKind::StringLit(rodeo.get_or_intern(&source[(start + 1)..pos])),
                span: Span { start, end, source_id }
            });
        }
    }
    Err(Diag::error()
        .with_message("Unterminated string")
        .with_labels(vec![
            Label::primary(source_id, start..end)
                .with_message("unterminated string")
        ]))
}

pub fn tokenize(
    source_id: usize,
    source: &str,
    rodeo: &mut lasso::Rodeo,
) -> Result<Vec<Token>, Diag> {
    let mut source_chars = source.char_indices().peekable();
    let mut tokens = Vec::new();

    while let Some((start, ch)) = source_chars.next() {
        match ch {
            ' ' | '\t' | '\n' | '\r' => continue,
            ';' => if let Some((_, ';')) = source_chars.peek() {
                for (_, ch) in source_chars.by_ref() {
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
            '+' => if let Some(&(pos, '=')) = source_chars.peek() {
                source_chars.next();
                tokens.push(Token {
                    kind: TokenKind::Reassign(ReassignmentOp::PlusEq),
                    span: Span { start, end: pos + '='.len_utf8(), source_id }
                });
            } else {
                tokens.push(Token {
                    kind: TokenKind::Operator(Operator::Plus),
                    span: Span { start, end: start + ch.len_utf8(), source_id }
                });
            },
            '-' => if let Some(&(pos, '=')) = source_chars.peek() {
                source_chars.next();
                tokens.push(Token {
                    kind: TokenKind::Reassign(ReassignmentOp::MinusEq),
                    span: Span { start, end: pos + '='.len_utf8(), source_id }
                });
            } else {
                tokens.push(Token {
                    kind: TokenKind::Operator(Operator::Minus),
                    span: Span { start, end: start + ch.len_utf8(), source_id }
                });
            },
            '*' => if let Some(&(pos, '=')) = source_chars.peek() {
                source_chars.next();
                tokens.push(Token {
                    kind: TokenKind::Reassign(ReassignmentOp::StarEq),
                    span: Span { start, end: pos + '='.len_utf8(), source_id }
                });
            } else {
                tokens.push(Token {
                    kind: TokenKind::Operator(Operator::Star),
                    span: Span { start, end: start + ch.len_utf8(), source_id }
                });
            },
            '/' => if let Some(&(pos, '=')) = source_chars.peek() {
                source_chars.next();
                tokens.push(Token {
                    kind: TokenKind::Reassign(ReassignmentOp::SlashEq),
                    span: Span { start, end: pos + '='.len_utf8(), source_id }
                });
            } else {
                tokens.push(Token {
                    kind: TokenKind::Operator(Operator::Slash),
                    span: Span { start, end: start + ch.len_utf8(), source_id }
                });
            },
            '%' => if let Some(&(pos, '=')) = source_chars.peek() {
                source_chars.next();
                tokens.push(Token {
                    kind: TokenKind::Reassign(ReassignmentOp::ModuloEq),
                    span: Span { start, end: pos + '='.len_utf8(), source_id }
                });
            } else {
                tokens.push(Token {
                    kind: TokenKind::Operator(Operator::Modulo),
                    span: Span { start, end: start + ch.len_utf8(), source_id }
                });
            },
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
            '=' => if let Some(&(pos, '=')) = source_chars.peek() {
                source_chars.next();
                tokens.push(Token {
                    kind: TokenKind::Operator(Operator::Eq),
                    span: Span { start, end: pos + '='.len_utf8(), source_id }
                });
            } else {
                tokens.push(Token {
                    kind: TokenKind::Reassign(ReassignmentOp::Assign),
                    span: Span { start, end: start + ch.len_utf8(), source_id }
                });
            },
            '!' => if let Some(&(pos, '=')) = source_chars.peek() {
                source_chars.next();
                tokens.push(Token {
                    kind: TokenKind::Operator(Operator::Ne),
                    span: Span { start, end: pos + '='.len_utf8(), source_id }
                });
            } else {
                tokens.push(Token {
                    kind: TokenKind::Operator(Operator::Bang),
                    span: Span { start, end: start + ch.len_utf8(), source_id }
                });
            },
            '>' => if let Some(&(pos, '=')) = source_chars.peek() {
                source_chars.next();
                tokens.push(Token {
                    kind: TokenKind::Operator(Operator::Ge),
                    span: Span { start, end: pos + '='.len_utf8(), source_id }
                });
            } else {
                tokens.push(Token {
                    kind: TokenKind::Operator(Operator::Gt),
                    span: Span { start, end: start + ch.len_utf8(), source_id }
                });
            },
            '<' => if let Some(&(pos, '=')) = source_chars.peek() {
                source_chars.next();
                tokens.push(Token {
                    kind: TokenKind::Operator(Operator::Le),
                    span: Span { start, end: pos + '='.len_utf8(), source_id }
                });
            } else {
                tokens.push(Token {
                    kind: TokenKind::Operator(Operator::Lt),
                    span: Span { start, end: start + ch.len_utf8(), source_id }
                });
            },
            '|' => if let Some(&(pos, '=')) = source_chars.peek() {
                source_chars.next();
                tokens.push(Token {
                    kind: TokenKind::Reassign(ReassignmentOp::PipeEq),
                    span: Span { start, end: pos + '='.len_utf8(), source_id }
                });
            } else {
                tokens.push(Token {
                    kind: TokenKind::Operator(Operator::Pipe),
                    span: Span { start, end: start + ch.len_utf8(), source_id }
                });
            },
            '&' => if let Some(&(pos, '=')) = source_chars.peek() {
                source_chars.next();
                tokens.push(Token {
                    kind: TokenKind::Reassign(ReassignmentOp::AmpersandEq),
                    span: Span { start, end: pos + '='.len_utf8(), source_id }
                });
            } else {
                tokens.push(Token {
                    kind: TokenKind::Operator(Operator::Ampersand),
                    span: Span { start, end: start + ch.len_utf8(), source_id }
                });
            },
            '^' => if let Some(&(pos, '=')) = source_chars.peek() {
                source_chars.next();
                tokens.push(Token {
                    kind: TokenKind::Reassign(ReassignmentOp::CaretEq),
                    span: Span { start, end: pos + '='.len_utf8(), source_id }
                });
            } else {
                tokens.push(Token {
                    kind: TokenKind::Operator(Operator::Caret),
                    span: Span { start, end: start + ch.len_utf8(), source_id }
                });
            },
            '.' => tokens.push(Token {
                kind: TokenKind::Dot,
                span: Span { start, end: start + ch.len_utf8(), source_id }
            }),
            '0'..='9' => tokens.push(lex_num(rodeo, source_id, &mut source_chars, source, start)),
            ch if ch.is_alphabetic() || ch == '_' => tokens.push(lex_ident(rodeo, source_id, &mut source_chars, source, start)),
            '"' => tokens.push(lex_str(rodeo, source_id, &mut source_chars, source, start)?),
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
