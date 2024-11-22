pub mod graph;

use graph::{
    node::Node,
    rule::Rule,
    searcher::{dfs, Searcher},
};

fn main() {
    let rule_list = vec![
        Rule::new(101, vec![Node::new(1), Node::new(2)], Node::new(3)),
        Rule::new(
            102,
            vec![Node::new(2), Node::new(3), Node::new(4)],
            Node::new(7),
        ),
        Rule::new(103, vec![Node::new(5), Node::new(6)], Node::new(4)),
        Rule::new(104, vec![Node::new(8), Node::new(31)], Node::new(3)),
        Rule::new(105, vec![Node::new(7), Node::new(9)], Node::new(14)),
        Rule::new(
            106,
            vec![Node::new(4), Node::new(10), Node::new(11)],
            Node::new(9),
        ),
        Rule::new(107, vec![Node::new(12), Node::new(13)], Node::new(11)),
        Rule::new(108, vec![Node::new(15), Node::new(21)], Node::new(33)),
        Rule::new(109, vec![Node::new(16), Node::new(17)], Node::new(15)),
        Rule::new(110, vec![Node::new(9), Node::new(21)], Node::new(14)),
        Rule::new(111, vec![Node::new(18), Node::new(32)], Node::new(9)),
        Rule::new(112, vec![Node::new(19), Node::new(20)], Node::new(21)),
    ];

    process_graph(
        rule_list,
        vec![
            Node::new(2),
            Node::new(3),
            Node::new(5),
            Node::new(6),
            Node::new(10),
            Node::new(11),
            Node::new(19),
            Node::new(20),
            // Node::new(9),
            // Node::new(16),
            // Node::new(17),
            // Node::new(19),
            // Node::new(20),
        ],
        Node::new(14),
    );
}

fn process_graph(rule_list: Vec<Rule>, from: Vec<Node>, to: Node) {
    let searcher = dfs::DFS {};

    println!("Search with DFS:");

    println!(
        "  Input nodes: {}",
        from.iter()
            .map(|x| x.number().to_string())
            .collect::<Vec<String>>()
            .join(",")
    );

    let (found, rules, nodes) = searcher.find_path(rule_list, from, to);

    if !found {
        println!("Solution not found");

        return;
    }

    println!("Soultion found");
    println!(
        "  Closed rules: {}",
        rules
            .iter()
            .map(|x| x.label().to_string())
            .collect::<Vec<String>>()
            .join(",")
    );
    println!(
        "  Closed nodes: {}",
        nodes
            .iter()
            .map(|x| x.number().to_string())
            .collect::<Vec<String>>()
            .join(",")
    );
}
