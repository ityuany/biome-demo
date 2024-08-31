use oxc_ast::AstKind;
use oxc_semantic::AstNode;

use crate::common::{Compat, CompatHandler, Support};

pub struct ArrowFunctionsCompat {
    pub compat: Compat,
}

impl Default for ArrowFunctionsCompat {
    fn default() -> Self {
        Self {
            compat: Compat {
                name: "arrow_functions".to_string(),
                description: "arrow function expressions".to_string(),
                tags: vec![
                    "web-features:arrow-functions".to_string(),
                    "web-features:snapshot:ecmascript-2015".to_string(),
                ],
                support: Support {
                    chrome: "45.0.0".to_string(),
                    chrome_android: "45".to_string(),
                    firefox: "22".to_string(),
                    firefox_android: "22".to_string(),
                    safari: "10".to_string(),
                    safari_ios: "10".to_string(),
                    edge: "12".to_string(),
                    node: "4.0.0".to_string(),
                    deno: "1.0".to_string(),
                },
            },
        }
    }
}

impl CompatHandler for ArrowFunctionsCompat {
    fn handle<'a>(&self, node: &AstNode<'a>) -> bool {
        matches!(node.kind(), AstKind::ArrowFunctionExpression(_))
    }

    fn get_compat(&self) -> &Compat {
        &self.compat
    }
}
