use arrow_functions::{ArrowFunctionsCompat, CompatHandler};
use common::Text;
use default_destructured_params::{CompatHandler as _, DefaultDestructuredParamsCompat};
use oxc_allocator::Allocator;
use oxc_ast::AstKind;
use oxc_parser::Parser;
use oxc_semantic::{AstNode, SemanticBuilder};
use oxc_span::{GetSpan, SourceType};
mod arrow_functions;
mod common;
mod default_destructured_params;

fn main() {
    let source = r#"
    function hello(){
        const fn1 = ({ a, b } = {}) => {
        const fn1 = ({ a, b } = {}) => {
            return a + b;
        };
            return a + b;
        };
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

    let mut vec: Vec<AstNode> = Vec::new();

    let nodes = semantic.nodes();

    for node in nodes.iter() {
        node.kind().span();
        if let Some(parent_id) = nodes.parent_id(node.id()) {
            let parent_node = nodes.get_node(parent_id);
            println!(
                "当前节点: {:?}, 父节点: {:?}",
                node.text(source),
                parent_node.text(source)
            );
        }

        // ArrowFunctionsCompat::default().handle(source, node, &mut vec);
        // DefaultDestructuredParamsCompat::default().handle(source, node, &mut vec);
    }

    for node in vec.iter() {
        println!("node: {:?}", node.kind().debug_name());
    }
}
