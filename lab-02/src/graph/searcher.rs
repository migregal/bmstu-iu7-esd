use super::{node::Node, rule::Rule};

pub mod bfs;

pub trait Searcher {
    fn find_path(self, rule_list: Vec<Rule>, from: Vec<Node>, to: Node) ->  (bool, Vec<Rule>, Vec<Node>);
}
