use super::{edge::Edge, node::Node};

pub mod bfs;
pub mod dfs;

pub trait Searcher {
    fn find_path(self, edge_list: Vec<Edge>, from: Node, to: Node) -> Option<Vec<Node>>;
}
