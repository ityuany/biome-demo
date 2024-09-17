use petgraph::{algo::is_cyclic_directed, graphmap::NodeTrait, prelude::DiGraphMap};

pub struct ModuleGraph<'a> {
    pub graph: DiGraphMap<&'a String, ()>,
    pub graph_meta: Vec<(String, String)>,
}

impl<'a> ModuleGraph<'a> {
    pub fn new(graph_meta: Vec<(String, String)>) -> Self {
        let mut graph = DiGraphMap::new();
        for (key, value) in graph_meta {
            graph.add_edge(&key, &value, ());
        }
        Self { graph, graph_meta }
    }

    pub fn detect_cycle(&self) -> bool {
        is_cyclic_directed(self.graph)
    }
}

pub fn create() -> ModuleGraph {
    let graph_meta = vec![
        ("file1".to_string(), "file2".to_string()),
        ("file12".to_string(), "file2".to_string()),
        ("file2".to_string(), "file3".to_string()),
        ("file3".to_string(), "file4".to_string()),
        ("file4".to_string(), "file1".to_string()),
    ];
    ModuleGraph::new(graph_meta)
}

fn main() {
    let graph = create();
    println!("{:?}", graph);
}
