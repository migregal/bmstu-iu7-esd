pub mod graph;

use graph::{
    edge::Edge,
    node::Node,
    searcher::{bfs, dfs, Searcher},
};

fn main() {
    println!("Search on first graph");
    println!("=====================");

    let edge_list = vec![
        Edge::new(Node::new(0), Node::new(1), 101, false),
        Edge::new(Node::new(1), Node::new(2), 102, false),
        Edge::new(Node::new(0), Node::new(3), 103, false),
        Edge::new(Node::new(3), Node::new(4), 104, false),
        Edge::new(Node::new(3), Node::new(5), 105, false),
        Edge::new(Node::new(6), Node::new(5), 106, false),
        Edge::new(Node::new(5), Node::new(4), 107, false),
        Edge::new(Node::new(2), Node::new(6), 108, false),
        Edge::new(Node::new(2), Node::new(4), 109, false),
    ];

    process_graph(edge_list, Node::new(0), Node::new(6));
}

fn process_graph(edge_list: Vec<Edge>, from: Node, to: Node) {
    let searcher = dfs::DFS {};

    println!("Search with DFS:");
    let res = searcher.find_path(edge_list.clone(), from, to);

    if res.is_none() {
        println!("\tNo path found;");
    } else {
        print!("\tSulution:\n\t\t");
        print_path(res.unwrap());
    }

    let searcher = bfs::BFS {};

    println!("Search with BFS:");
    let res = searcher.find_path(edge_list.clone(), from, to);

    if res.is_none() {
        println!("\tNo path found;");
    } else {
        print!("\tSulution:\n\t\t");
        print_path(res.unwrap());
    }
}

fn print_path(path: Vec<Node>) {
    for (idx, node) in path.iter().enumerate() {
        print!("{}", node);
        if idx != path.len() - 1 {
            print!(" -> ")
        }
    }
    println!();
}
