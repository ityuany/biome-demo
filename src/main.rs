use biome_js_parser::{parse, JsParserOptions};
use biome_js_syntax::{AnyJsRoot, JsFileSource, JsFormalParameter, JsLanguage, JsSyntaxKind};
use biome_rowan::{AstNode, SyntaxNode};

fn count_arrow_functions(item: &SyntaxNode<JsLanguage>, vec: &mut Vec<SyntaxNode<JsLanguage>>) {
    if item.kind() == JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION {
        vec.push(item.clone());
    }
}

fn count_default_parameters_destructured(
    item: &SyntaxNode<JsLanguage>,
    vec: &mut Vec<SyntaxNode<JsLanguage>>,
) {
    if item.kind() == JsSyntaxKind::JS_FORMAL_PARAMETER {
        if let Some(param) = JsFormalParameter::cast(item.clone()) {
            if let Ok(binding) = param.binding() {
                if matches!(
                    binding.syntax().kind(),
                    JsSyntaxKind::JS_OBJECT_BINDING_PATTERN // 匹配对象解构模式
                        | JsSyntaxKind::JS_ARRAY_BINDING_PATTERN // 或数组解构模式
                ) {
                    println!("=-->{:?} {:?}", item.text(), item.text_range());
                    vec.push(item.clone());
                }
            }
        }
    }
}

fn main() {
    let source_code = r#"
        function example({ a = 1, b = 2 } = {}) {}
        const arrowFunc = ({ x = 10, y = 20 } = {}) => {
            const arrowFunc2 = ({ x = 10, y = 20 } = {}) => {};
        };
        function another([first = 'default', second = 0] = []) {}
    "#;

    let parse_result = parse(
        source_code,
        JsFileSource::js_module(),
        JsParserOptions::default(),
    );
    let root: AnyJsRoot = parse_result.tree();

    let mut vec = Vec::new();

    let nodes = root.syntax().descendants();

    for node in nodes {
        count_default_parameters_destructured(&node, &mut vec);
        count_arrow_functions(&node, &mut vec);
    }

    println!("=-->{:?}", vec.len());
}
