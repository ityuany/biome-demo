use oxc_allocator::Allocator;
use oxc_ast::{
    ast::{ArrowFunctionExpression, BindingPattern, FormalParameter},
    AstKind,
};
use oxc_parser::Parser;
use oxc_semantic::{Semantic, SemanticBuilder};
use oxc_span::SourceType;

fn find_arrow_functions<'a>(
    source: &str,
    semantic: &Semantic<'a>,
    vec: &mut Vec<AstKind<'a>>,
) -> Vec<String> {
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

fn find_default_destructured_params<'a>(
    source: &str,
    semantic: &Semantic<'a>,
    vec: &mut Vec<AstKind<'a>>,
) -> Vec<String> {
    let mut default_destructured_params = Vec::new();

    for node in semantic.nodes().iter() {
        vec.push(node.kind());
        if let AstKind::ArrowFunctionExpression(arrow_func) = node.kind() {
            check_params(arrow_func, source, &mut default_destructured_params);
            println!("--> {:?} \n", vec);
        }
        vec.pop();
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

fn main() {
    let source = r#"
    hello(){
        const fn1 = ({ a, b } = {}) => a + b;
    }
      
  "#;

    let allocator = Allocator::default();
    let source_type = SourceType::default();

    let ret = Parser::new(&allocator, source, source_type).parse();
    let program = allocator.alloc(ret.program);
    let semantic = SemanticBuilder::new(source, source_type)
        .build(program)
        .semantic;

    let mut stack: Vec<AstKind> = Vec::new();

    let params = find_default_destructured_params(source, &semantic, &mut stack);
    let arrow_functions = find_arrow_functions(source, &semantic, &mut stack);

    println!("Found {} default destructured params:", params.len());
    for (index, param) in params.iter().enumerate() {
        println!("{}. {}", index + 1, param);
    }

    println!("Found {} arrow functions:", arrow_functions.len());
    for (index, func) in arrow_functions.iter().enumerate() {
        println!("{}. {}", index + 1, func);
    }
}
