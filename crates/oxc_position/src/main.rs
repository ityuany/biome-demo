use oxc_allocator::Allocator;

use oxc_ast::AstKind;
use oxc_parser::Parser;
use oxc_semantic::SemanticBuilder;

use oxc_span::SourceType;
use ropey::Rope;
use tower_lsp::lsp_types::Position;

use oxc_span::GetSpan;

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
            let start_position = offset_to_position(span.start as usize, &source).unwrap();
            let end_position = offset_to_position(span.end as usize, &source).unwrap();
            println!("start_position: {:?}", start_position);
            println!("end_position: {:?}", end_position);
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
