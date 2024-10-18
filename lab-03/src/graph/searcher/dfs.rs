use std::collections::HashSet;
use std::vec::Vec;

use crate::graph::node::Node;
use crate::graph::rule::Rule;

use super::Searcher;

#[derive(Clone, Copy)]
pub struct DFS {}

impl DFS {
    fn child_search(
        rules_list: &mut Vec<Rule>,
        to: Node,
        opened_nodes: &mut Vec<Node>,
        closed_nodes: &mut HashSet<Node>,
        opened_rules: &mut Vec<Rule>,
    ) -> (bool, bool) {
        let (mut have_children, mut solution_found) = (false, false);

        for rule in rules_list.iter_mut().filter(|x| !x.used()) {
            let cur_node = opened_nodes.last().unwrap().to_owned();

            if rule.to() != cur_node {
                continue;
            }

            rule.set_used(true);
            opened_rules.push(rule.to_owned());
            have_children = true;

            let prev_len = opened_nodes.len();
            opened_nodes.extend(rule.from().iter().filter(|x| !closed_nodes.contains(x)));
            if prev_len == opened_nodes.len() {
                solution_found =
                    DFS::label(to, opened_nodes, closed_nodes, opened_rules).is_some_and(|x| x);
            }

            break;
        }

        return (solution_found, have_children);
    }

    fn label(
        to: Node,
        opened_nodes: &mut Vec<Node>,
        closed_nodes: &mut HashSet<Node>,
        opened_rules: &mut Vec<Rule>,
    ) -> Option<bool> {
        loop {
            let (_, node) = (opened_rules.pop()?, opened_nodes.pop()?);

            closed_nodes.insert(node);

            if node == to {
                return Some(true);
            }

            let (cur_rule, cur_node) = (
                opened_rules.last()?.to_owned(),
                opened_nodes.last()?.to_owned(),
            );
            if cur_rule.to() != cur_node {
                break;
            }
        }

        return Some(false);
    }

    fn backtracking(opened_nodes: &mut Vec<Node>, opened_rules: &mut Vec<Rule>) {
        let (_, rule) = (opened_nodes.pop(), opened_rules.pop());

        opened_nodes.remove(
            opened_nodes
                .iter()
                .position(|x| rule.clone().unwrap().from().contains(x))
                .unwrap(),
        );
    }
}

impl Searcher for DFS {
    fn find_path(
        self,
        rules: Vec<Rule>,
        from: Vec<Node>,
        to: Node,
    ) -> (bool, Vec<Rule>, Vec<Node>) {
        let (mut opened_nodes, mut closed_nodes) = (vec![to], HashSet::from_iter(from));

        let mut opened_rules = Vec::<Rule>::new();

        let mut mut_rules_list = rules.clone().to_owned();
        let (mut solution_found, mut have_children) = (false, true);

        while have_children && !solution_found {
            (solution_found, have_children) = DFS::child_search(
                &mut mut_rules_list,
                to,
                &mut opened_nodes,
                &mut closed_nodes,
                &mut opened_rules,
            );

            if solution_found {
                break;
            }

            if !have_children {
                if opened_nodes.len() == 1 && opened_nodes[0] == to {
                    break;
                }

                if opened_nodes.len() > 0 {
                    DFS::backtracking(&mut opened_nodes, &mut opened_rules);
                    have_children = true;
                }
            }
        }

        // sort results
        mut_rules_list.sort_by_key(|x| x.label());
        let mut nodes = Vec::from_iter(closed_nodes);
        nodes.sort_by_key(|x| x.number());

        return (solution_found, mut_rules_list, nodes);
    }
}
