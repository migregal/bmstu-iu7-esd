use std::collections::HashSet;
use std::vec::Vec;

use crate::graph::node::Node;
use crate::graph::rule::Rule;

use super::Searcher;

#[derive(Clone, Copy)]
pub struct BFS {}

impl BFS {
    fn parent_search(
        rules_list: &mut Vec<Rule>,
        to: Node,
        closed_nodes: &mut HashSet<Node>,
    ) -> (bool, bool) {
        let mut have_parent = false;

        for rule in rules_list.iter_mut().filter(|x| !x.used()) {
            if !rule.from().iter().all(|item| closed_nodes.contains(item)) {
                continue;
            }

            rule.set_used(true);
            closed_nodes.insert(rule.to());
            have_parent = true;

            if rule.to() == to {
                return (true, have_parent);
            }
        }

        return (false, have_parent);
    }
}

impl Searcher for BFS {
    fn find_path(
        self,
        rules: Vec<Rule>,
        from: Vec<Node>,
        to: Node,
    ) -> (bool, Vec<Rule>, Vec<Node>) {
        let mut closed_nodes = HashSet::<Node>::from_iter(from);
        let mut mut_rules_list = rules.clone().to_owned();
        let (mut solution_found, mut have_parent) = (false, true);

        while have_parent && !solution_found {
            (solution_found, have_parent) =
                BFS::parent_search(&mut mut_rules_list, to, &mut closed_nodes);
        }

        // sort results
        mut_rules_list.sort_by_key(|x| x.label());
        let mut nodes = Vec::from_iter(closed_nodes);
        nodes.sort_by_key(|x| x.number());

        return (solution_found, mut_rules_list, nodes);
    }
}
