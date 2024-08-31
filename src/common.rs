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

pub struct Compat {
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub support: Support,
}

pub struct Support {
    pub chrome: String,
    pub chrome_android: String,
    pub firefox: String,
    pub firefox_android: String,
    pub safari: String,
    pub safari_ios: String,
    pub edge: String,
    pub node: String,
    pub deno: String,
}

pub trait CompatHandler {
    fn handle<'a>(&self, node: &AstNode<'a>) -> bool;

    fn get_compat(&self) -> &Compat;
}
