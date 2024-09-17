use oxc_allocator::Allocator;

use oxc_ast::AstKind;
use oxc_parser::Parser;
use oxc_semantic::SemanticBuilder;
use oxc_span::GetSpan;
use oxc_span::SourceType;

fn main() {
    let source = r#"
        function hello(){
            const fn1 = ({ a, b } = {}) => {
            const fn1 = ({ a, b }) => {
                return a + b;
            };
                return a + b;
            };
        }
      "#;

    let allocator = Allocator::default();
    let source_type = SourceType::default();

    let ret = Parser::new(&allocator, &source, source_type).parse();
    let program = allocator.alloc(ret.program);
    let semantic = SemanticBuilder::new(&source, source_type)
        .build(program)
        .semantic;

    let nodes = semantic.nodes();

    for node in nodes.iter() {
        if matches!(node.kind(), AstKind::Function(_)) {
            let span = GetSpan::span(node);
            println!("span: {:?}", span);
        }
    }
}
