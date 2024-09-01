use crate::common::{Compat, CompatHandler, Support};
use oxc_ast::{ast::BindingPatternKind, AstKind};
use oxc_semantic::AstNode;

pub struct DefaultDestructuredParamsCompat {
    pub compat: Compat,
}

impl Default for DefaultDestructuredParamsCompat {
    fn default() -> Self {
        Self {
            compat: Compat {
                name: "default_parameters_destructured_parameter_with_default_value_assignment".to_string(),
                description: "destructured parameter with default value assignment".to_string(),
            tags: vec![
                "web-features:default-parameters-destructured-parameter-with-default-value-assignment".to_string(),
                "web-features:snapshot:ecmascript-2015".to_string()
            ],
            support: Support {
                chrome: "49.0.0".to_string(),
                chrome_android: "49".to_string(),
                    firefox: "41".to_string(),
                    firefox_android: "41".to_string(),
                    safari: "10".to_string(),
                    safari_ios: "10".to_string(),
                edge: "14".to_string(),
                node: "6.0.0".to_string(),
                deno: "1.0".to_string(),
            }
            }
        }
    }
}

impl CompatHandler for DefaultDestructuredParamsCompat {
    fn handle<'a>(&self, node: &AstNode<'a>) -> bool {
        if let AstKind::FormalParameter(param) = node.kind() {
            matches!(
                &param.pattern.kind,
                BindingPatternKind::AssignmentPattern(_)
            )
        } else {
            false
        }
    }

    fn get_compat(&self) -> &Compat {
        &self.compat
    }
}
