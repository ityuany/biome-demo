use oxc_allocator::Allocator;
use oxc_ast::{
    ast::{ArrowFunctionExpression, BindingPattern, FormalParameter},
    AstKind,
};
use oxc_parser::Parser;
use oxc_semantic::SemanticBuilder;
use oxc_span::SourceType;

fn find_arrow_functions(source: &str) -> Vec<String> {
    let allocator = Allocator::default();
    let source_type = SourceType::default();

    let ret = Parser::new(&allocator, source, source_type).parse();
    let program = allocator.alloc(ret.program);
    let semantic = SemanticBuilder::new(source, source_type)
        .build(program)
        .semantic;

    let mut arrow_functions = Vec::new();
    for node in semantic.nodes().iter() {
        if let AstKind::ArrowFunctionExpression(arrow_func) = node.kind() {
            let span = arrow_func.span;
            let func_text = &source[span.start as usize..span.end as usize];
            arrow_functions.push(func_text.to_string());
        }
    }

    arrow_functions
}

fn find_default_destructured_params(source: &str) -> Vec<String> {
    let allocator = Allocator::default();
    let source_type = SourceType::default();

    let ret = Parser::new(&allocator, source, source_type).parse();
    let program = allocator.alloc(ret.program);
    let semantic = SemanticBuilder::new(source, source_type)
        .build(program)
        .semantic;

    let mut default_destructured_params = Vec::new();

    for node in semantic.nodes().iter() {
        if let AstKind::ArrowFunctionExpression(arrow_func) = node.kind() {
            check_params(arrow_func, source, &mut default_destructured_params);
        }
    }

    default_destructured_params
}

fn check_params(arrow_func: &ArrowFunctionExpression, source: &str, results: &mut Vec<String>) {
    for param in &arrow_func.params.items {
        if let FormalParameter {
            span,
            decorators,
            pattern,
            accessibility,
            readonly,
            r#override,
        } = param
        {
            let span = param.span;
            let param_text = &source[span.start as usize..span.end as usize];
            results.push(param_text.to_string());
        }
    }
}

// fn main() {
//     let source = r#"
//       const outer = (x) => {
//           const inner = (y) => y * 2;
//           return inner(x) + 1;
//       };

//       let multiply = (a, b) => a * b;

//       const nested = () => () => "I'm deeply nested!";
//   "#;

//     let arrow_functions = find_arrow_functions(source);

//     println!("Found {} arrow function(s):", arrow_functions.len());
//     for (index, func) in arrow_functions.iter().enumerate() {
//         println!("{}. {}", index + 1, func);
//     }
// }

fn main() {
    let source = r#"
      const fn1 = ({ a, b } = {}) => a + b;
      const fn2 = ({ x = 1, y = 2 } = {}) => x * y;
      const fn3 = (normal, { z = 3 } = {}) => normal + z;
  "#;

    let params = find_default_destructured_params(source);

    println!("Found {} default destructured params:", params.len());
    for (index, param) in params.iter().enumerate() {
        println!("{}. {}", index + 1, param);
    }
}
