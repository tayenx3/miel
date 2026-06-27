mod symbol;
mod imports {
    pub use super::super::ty::*;
    pub use super::super::inst::*;
    pub use super::super::callable::*;
    pub use super::super::builder::*;
    pub use super::super::module::*;
}

use imports::*;
use crate::common::{Context, Diag, Label};
use crate::parser::ast::*;

pub struct Codegen<'a> {
    ctx: Context<'a>,
    pub module: IrModule<'a>,
}

impl<'a> Codegen<'a> {
    pub fn new(name: &str, ctx: Context<'a>) -> Self {
        Self {
            ctx: Context::clone(&ctx),
            module: IrModule::new(name, ctx),
        }
    }

    pub fn generate(&mut self, ast: &Ast) -> Result<(), Diag> {
        Ok(())
    }

    fn collect_constant(&mut self, node: &Node) -> Result<(), Diag> {
        match &node.kind {
            NodeKind::Semi(stmt) => self.collect_constant(stmt),
            NodeKind::ConstDecl { name, ty, expr } => {
                Ok(())
            },
            _ => Err(Diag::error()
                .with_message("Expected root-level item")
                .with_labels(vec![
                    Label::primary(node.span.source_id, node.span.start..node.span.end)
                        .with_message("expression is not a valid root-level item")
                ]))
        }
    }
}