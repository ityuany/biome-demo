use petgraph::algo::{is_cyclic_directed, kosaraju_scc};
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

// 检查是否有循环依赖
fn detect_cycle(graph: &DiGraphMap<&str, ()>) -> bool {
    is_cyclic_directed(graph)
}

fn detect_cycle_files<'a>(graph: &'a DiGraphMap<&'a str, ()>) -> Vec<Vec<&'a str>> {
    kosaraju_scc(graph)
        .into_iter()
        .filter(|scc| scc.len() > 1 || (scc.len() == 1 && graph.contains_edge(scc[0], scc[0])))
        .collect()
}

// 获取依赖某个文件的所有文件
fn get_dependents<'a>(graph: &'a DiGraphMap<&'a str, ()>, file: &'a str) -> HashSet<&'a str> {
    graph
        .neighbors_directed(file, petgraph::Direction::Incoming)
        .collect()
}

fn main() {
    let graph = create_graph();

    if detect_cycle(&graph) {
        println!("发现循环依赖！");
    } else {
        println!("没有循环依赖。");
    }

    let cycles = detect_cycle_files(&graph);
    if !cycles.is_empty() {
        println!("发现以下循环依赖：");
        for (i, cycle) in cycles.iter().enumerate() {
            println!("循环 {}: {:?}", i + 1, cycle);
        }
    } else {
        println!("没有循环依赖。");
    }

    // 获取依赖 "file2" 的文件
    let dependents = get_dependents(&graph, "file2");
    println!("依赖 'file2' 的文件: {:?}", dependents);
}
