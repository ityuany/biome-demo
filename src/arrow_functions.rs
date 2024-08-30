use crate::common::Text;
use oxc_ast::{ast::BindingPatternKind, AstKind};
use oxc_semantic::AstNode;
pub trait CompatHandler {
    fn handle<'a>(&self, source: &str, node: &AstNode<'a>, usage: &mut Vec<AstNode<'a>>);
}
pub struct ArrowFunctionsCompat {
    pub name: String,
}

impl Default for ArrowFunctionsCompat {
    fn default() -> Self {
        Self {
            name: "default_destructured_params".to_string(),
        }
    }
}

impl CompatHandler for ArrowFunctionsCompat {
    fn handle<'a>(&self, source: &str, node: &AstNode<'a>, usage: &mut Vec<AstNode<'a>>) {
        match node.kind() {
            AstKind::ArrowFunctionExpression(_arrow_func) => {
                println!("func_text: {:?}", node.text(source));
                usage.push(node.clone());
            }
            _ => {}
        }
    }
}
