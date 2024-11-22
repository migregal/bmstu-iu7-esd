use std::collections::HashSet;
use std::vec::Vec;

use crate::graph::edge::Edge;
use crate::graph::node::Node;

use super::Searcher;

#[derive(Clone, Copy)]
pub struct DFS {}

impl DFS {
    fn sample_search(
        self,
        edge_list: &mut Vec<Edge>,
        to: Node,
        opened_nodes: &mut Vec<Node>,
        closed_nodes: &mut HashSet<Node>,
    ) -> (bool, bool) {
        let mut have_children = false;
        let mut cur_node = opened_nodes.last().unwrap().to_owned();

        for edge in edge_list.iter_mut().filter(|x| !x.used()) {
            if edge.from() != cur_node {
                continue;
            }

            if opened_nodes.contains(&edge.to()) || closed_nodes.contains(&edge.to()) {
                continue;
            }

            edge.set_used(true);
            opened_nodes.push(edge.to());
            cur_node = edge.to();
            have_children = true;

            if edge.to() == to {
                return (true, have_children);
            }
        }

        return (false, have_children);
    }
}

impl Searcher for DFS {
    fn find_path(self, edge_list: Vec<Edge>, from: Node, to: Node) -> Option<Vec<Node>> {
        let (mut opened_nodes, mut closed_nodes) = (vec![from], HashSet::<Node>::new());

        let mut mut_edge_list = edge_list.to_owned();

        let (mut solution_found, mut have_children) = (false, true);

        while have_children && !solution_found {
            (solution_found, have_children) =
                self.sample_search(&mut mut_edge_list, to, &mut opened_nodes, &mut closed_nodes);
            if solution_found {
                break;
            }

            if !have_children && opened_nodes.len() > 1 {
                closed_nodes.insert(opened_nodes.pop().unwrap());
                have_children = true
            }
        }

        if !solution_found {
            return None;
        }

        return Some(opened_nodes);
    }
}
