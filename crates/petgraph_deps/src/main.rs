use petgraph::graphmap::DiGraphMap;
use std::collections::HashSet;

// 创建依赖图
fn create_graph() -> DiGraphMap<&'static str, ()> {
    let mut graph = DiGraphMap::new();
    graph.add_edge("file1", "file2", ());
    graph.add_edge("file12", "file2", ());
    graph.add_edge("file2", "file3", ());
    graph.add_edge("file3", "file4", ());
    // 加入一个循环依赖：file4 -> file1
    graph.add_edge("file4", "file1", ());
    graph
}

// 获取依赖某个文件的所有文件
fn get_dependents<'a>(graph: &'a DiGraphMap<&'a str, ()>, file: &'a str) -> HashSet<&'a str> {
    graph
        .neighbors_directed(file, petgraph::Direction::Incoming)
        .collect()
}

fn main() {
    let graph = create_graph();

    // 获取依赖 "file2" 的文件
    let dependents = get_dependents(&graph, "file2");
    println!("依赖 'file2' 的文件: {:?}", dependents);
}
