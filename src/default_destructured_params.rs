use crate::common::Text;
use oxc_ast::{ast::BindingPatternKind, AstKind};
use oxc_semantic::AstNode;
pub trait CompatHandler {
    fn handle<'a>(&self, source: &str, node: &AstNode<'a>, usage: &mut Vec<AstNode<'a>>);
}
pub struct DefaultDestructuredParamsCompat {
    pub name: String,
}

impl Default for DefaultDestructuredParamsCompat {
    fn default() -> Self {
        Self {
            name: "default_destructured_params".to_string(),
        }
    }
}

impl CompatHandler for DefaultDestructuredParamsCompat {
    fn handle<'a>(&self, source: &str, node: &AstNode<'a>, usage: &mut Vec<AstNode<'a>>) {
        if let AstKind::FormalParameter(param) = node.kind() {
            if matches!(
                &param.pattern.kind,
                BindingPatternKind::AssignmentPattern(_)
            ) {
                println!("pat: {:?}", node.text(source));
                usage.push(node.clone());
            }
        }
    }
}
