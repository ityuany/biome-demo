use std::str::FromStr;

use arrow_functions::ArrowFunctionsCompat;
use common::{CompatHandler, Text};
use default_destructured_params::DefaultDestructuredParamsCompat;
use oxc_allocator::Allocator;

use oxc_parser::Parser;
use oxc_semantic::{AstNode, SemanticBuilder};
use oxc_span::SourceType;
use semver::{Version, VersionReq};
mod arrow_functions;
mod common;
mod default_destructured_params;

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

    let ret = Parser::new(&allocator, source, source_type).parse();
    let program = allocator.alloc(ret.program);
    let semantic = SemanticBuilder::new(source, source_type)
        .build(program)
        .semantic;

    let mut vec: Vec<AstNode> = Vec::new();

    let nodes = semantic.nodes();

    let compat_handlers: Vec<Box<dyn CompatHandler>> = vec![
        Box::new(ArrowFunctionsCompat::default()),
        Box::new(DefaultDestructuredParamsCompat::default()),
    ];

    let req = VersionReq::parse(">=45").unwrap();

    let u = compat_handlers
        .iter()
        .filter(|item| {
            let chrome = &item.get_compat().support.chrome;
            let x = Version::from_str(&chrome).unwrap();
            req.matches(&x)
        })
        .collect::<Vec<_>>();

    for node in nodes.iter() {
        if let Some(parent_id) = nodes.parent_id(node.id()) {
            let parent_node = nodes.get_node(parent_id);
            println!(
                "当前节点: {:?}, 父节点: {:?}",
                node.text(source),
                parent_node.text(source)
            );
        }

        for handler in &u {
            if handler.handle(node) {
                vec.push(node.clone());
            }
        }
    }

    for node in vec.iter() {
        println!("node: {:?}", node.kind().debug_name());
    }
}
