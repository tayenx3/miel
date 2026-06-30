//! Commonly used error diagnostics

use colored::Colorize;
use crate::common::{Diag, Label, Span};

pub fn non_bool_if_condition(span: Span) -> Diag {
    Diag::error()
        .with_message("Type mismatch")
        .with_labels(vec![
            Label::primary(span.source_id, span.start..span.end)
                .with_message(format!("expected `bool`"))
        ]).with_notes(vec![format!(
            "{}: `if` conditions must be of type `bool`",
            "note".bright_cyan().bold().underline()
        )])
}

pub fn type_mismatch(span: &Span, expected: &str, found: &str, notes: &[&str]) -> Diag {
    Diag::error()
        .with_message("Type mismatch")
        .with_labels(vec![
            Label::primary(span.source_id, span.start..span.end)
                .with_message(format!("expected `{expected}`, found `{found}`"))
        ]).with_notes(
            notes.iter()
                .map(|s| format!("{}: {s}", "note".bright_cyan().bold().underline()))
                .collect()
        )
}

pub fn unknown_ident(span: &Span, ident: &str, notes: &[&str]) -> Diag {
    Diag::error()
        .with_message("Unknown identifier")
        .with_labels(vec![
            Label::primary(span.source_id, span.start..span.end)
                .with_message(format!("`{ident}` not in scope"))
        ]).with_notes(
            notes.iter()
                .map(|s| format!("{}: {s}", "note".bright_cyan().bold().underline()))
                .collect()
        )
}