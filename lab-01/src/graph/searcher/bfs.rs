use std::collections::{HashMap, LinkedList};
use std::vec::Vec;

use crate::graph::edge::Edge;
use crate::graph::node::Node;

use super::Searcher;

#[derive(Clone, Copy)]
pub struct BFS {}

impl BFS {
    fn sample_search(
        edge_list: &mut Vec<Edge>,
        to: Node,
        opened_nodes: &mut LinkedList<Node>,
        closed_nodes: &mut Vec<Node>,
        result_path: &mut HashMap<Node, Node>,
    ) -> (bool, bool) {
        let mut have_children = false;
        let cur_node = opened_nodes.front().unwrap().to_owned();

        for edge in edge_list
            .iter_mut()
            .filter(|x| !x.used())
            .filter(|x| x.from() == cur_node)
        {
            if opened_nodes.contains(&edge.to()) || closed_nodes.contains(&edge.to()) {
                continue;
            }

            edge.set_used(true);
            opened_nodes.push_back(edge.to());
            result_path.insert(edge.to(), edge.from());
            have_children = true;

            if edge.to() == to {
                return (true, have_children);
            }
        }

        return (false, have_children);
    }

    fn build_path(from: Node, to: Node, result_path: HashMap<Node, Node>) -> Vec<Node> {
        let (mut cur, mut res) = (to, LinkedList::<Node>::from([to]));

        while cur != from {
            cur = result_path.get(&cur).unwrap().clone();
            res.push_front(cur);
        }

        return Vec::from_iter(res);
    }
}

impl Searcher for BFS {
    fn find_path(self, edge_list: Vec<Edge>, from: Node, to: Node) -> Option<Vec<Node>> {
        let (mut opened_nodes, mut closed_nodes) =
            (LinkedList::<Node>::from([from]), Vec::<Node>::new());

        let mut mut_edge_list = edge_list.to_owned();

        let mut result_path = HashMap::<Node, Node>::new();

        let (mut solution_found, mut have_children) = (false, true);

        while have_children && !solution_found {
            (solution_found, have_children) = BFS::sample_search(
                &mut mut_edge_list,
                to,
                &mut opened_nodes,
                &mut closed_nodes,
                &mut result_path,
            );
            if solution_found {
                break;
            }

            closed_nodes.push(opened_nodes.pop_front().unwrap());
            if !opened_nodes.is_empty() {
                have_children = true
            }
        }

        if !solution_found {
            return None;
        }

        return Some(BFS::build_path(from, to, result_path));
    }
}
