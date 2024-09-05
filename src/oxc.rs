use std::{fs, str::FromStr};

use arrow_functions::ArrowFunctionsCompat;
use common::{CompatHandler, Text};
use default_destructured_params::DefaultDestructuredParamsCompat;
use oxc_allocator::Allocator;

use oxc_ast::AstKind;
use oxc_parser::Parser;
use oxc_semantic::{AstNode, SemanticBuilder};
use oxc_sourcemap::SourceMap;
use oxc_span::SourceType;
use semver::{Version, VersionReq};
use tower_lsp::lsp_types::Position;
mod arrow_functions;
mod common;
use ropey::Rope;
mod default_destructured_params;
use oxc_span::GetSpan;

fn main() {
    //     let source = r#"
    //     function hello(){
    //         const fn1 = ({ a, b } = {}) => {
    //         const fn1 = ({ a, b }) => {
    //             return a + b;
    //         };
    //             return a + b;
    //         };
    //     }
    //   "#;

    let source = fs::read_to_string("/Users/ityuany/GitRepository/wms/dist/statics/src_lego_lessCoding_me-json_basic-_-abc-plus_js.418fc0f1.chunk.js").unwrap();
    let source_map_json = fs::read_to_string("/Users/ityuany/GitRepository/wms/dist/statics/src_lego_lessCoding_me-json_basic-_-abc-plus_js.418fc0f1.chunk.js.map").unwrap();

    let sourcemap = SourceMap::from_json_string(&source_map_json).unwrap();

    let lookup_table = sourcemap.generate_lookup_table();

    let allocator = Allocator::default();
    let source_type = SourceType::default();

    let ret = Parser::new(&allocator, &source, source_type).parse();
    let program = allocator.alloc(ret.program);
    let semantic = SemanticBuilder::new(&source, source_type)
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
        // if let Some(parent_id) = nodes.parent_id(node.id()) {
        //     let parent_node = nodes.get_node(parent_id);

        //     println!(
        //         "当前节点: {:?}, 父节点: {:?}",
        //         node.text(source),
        //         parent_node.text(source)
        //     );
        // }

        // for handler in &u {
        //     if handler.handle(node) {
        //         vec.push(node.clone());
        //     }
        // }
        if matches!(node.kind(), AstKind::Function(_)) {
            vec.push(node.clone());
        }
    }

    for node in vec.iter() {
        let span = node.kind().span();
        let start_position = offset_to_position(span.start as usize, &source).unwrap();
        let end_position = offset_to_position(span.end as usize, &source).unwrap();
        println!(
            "node: {:?}, start_position: {:?}, end_position: {:?}",
            node.kind().debug_name(),
            start_position,
            end_position
        );
        if let Some(token) = sourcemap.lookup_source_view_token(
            &lookup_table,
            start_position.line,
            start_position.character,
        ) {
            println!("-----------------------------------");
            println!("{:?}", token.get_source());
            println!(" {:?}", token.get_source_content());
        } else {
            println!("Token not found");
        }
    }
}

fn offset_to_position(offset: usize, source_text: &str) -> Option<Position> {
    let rope = Rope::from_str(source_text);
    let line = rope.try_byte_to_line(offset).ok()?;
    let first_char_of_line = rope.try_line_to_char(line).ok()?;
    // Original offset is byte, but Rope uses char offset
    let offset = rope.try_byte_to_char(offset).ok()?;
    let column = offset - first_char_of_line;
    Some(Position::new(line as u32, column as u32))
}
