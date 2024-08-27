use biome_js_parser::{parse, JsParserOptions};
use biome_js_semantic::{semantic_model, SemanticModelOptions};
use biome_js_syntax::{AnyJsRoot, JsFileSource, JsFormalParameter, JsSyntaxKind};
use biome_rowan::AstNode;

fn count_arrow_functions(source_code: &str) -> usize {
    let parse_result = parse(
        source_code,
        JsFileSource::js_module(),
        JsParserOptions::default(),
    );
    let root: AnyJsRoot = parse_result.tree();

    let semantic_model = semantic_model(&root, SemanticModelOptions::default());

    root.syntax()
        .descendants()
        .filter(|node| node.kind() == JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION)
        .count()
}

fn count_default_parameters_destructured(source_code: &str) -> usize {
    let parse_result = parse(
        source_code,
        JsFileSource::js_module(),
        JsParserOptions::default(),
    );
    let root: AnyJsRoot = parse_result.tree();

    root.syntax()
        .descendants() // 获取语法树的所有后代节点
        .filter(|node| node.kind() == JsSyntaxKind::JS_FORMAL_PARAMETER) // 筛选出形式参数节点
        .filter_map(|node| JsFormalParameter::cast(node)) // 将节点转换为JsFormalParameter类型
        .filter(|param| {
            if let Ok(binding) = param.binding() {
                // 获取参数的绑定
                matches!(
                    binding.syntax().kind(),
                    JsSyntaxKind::JS_OBJECT_BINDING_PATTERN // 匹配对象解构模式
                        | JsSyntaxKind::JS_ARRAY_BINDING_PATTERN // 或数组解构模式
                )
            } else {
                false
            }
        })
        .filter(|param| param.initializer().is_some()) // 筛选出有默认值的参数
        .count() // 计算符合条件的参数数量
}

fn main() {
    // let source_code = r#"
    //     const add = (a, b) => a + b;
    //     const multiply = (x, y) => {
    //         return x * y;
    //     };
    //     [1, 2, 3].map(n => n * 2);
    // "#;

    // let arrow_function_count = count_arrow_functions(source_code);
    // println!("箭头函数的数量: {}", arrow_function_count);

    let source_code = r#"
        function example({ a = 1, b = 2 } = {}) {}
        const arrowFunc = ({ x = 10, y = 20 } = {}) => {
            const arrowFunc2 = ({ x = 10, y = 20 } = {}) => {};
        };
        function another([first = 'default', second = 0] = []) {}
    "#;

    let count = count_default_parameters_destructured(source_code);
    println!("默认参数中的解构参数数量: {}", count);
}
