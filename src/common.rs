use oxc_ast::AstKind;
use oxc_semantic::AstNode;
use oxc_span::{GetSpan, Span};

pub trait Text {
    fn text(&self, source: &str) -> String;
}

impl<'a> Text for AstNode<'a> {
    fn text(&self, source: &str) -> String {
        let span = self.kind().span();
        source[span.start as usize..span.end as usize].to_string()
    }
}
