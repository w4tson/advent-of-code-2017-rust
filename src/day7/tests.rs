use utils::file::read_puzzle_input;
use day7::build_graph;
use std::collections::HashMap;
use day7::part1::Node;
use std::collections::HashSet;
use std::iter::FromIterator;


#[test]
fn example() {
    
    let graph = build_graph(read_puzzle_input("aoc7-example"));
    let connected_nodes: HashMap<&String, &Node> = graph.graph.iter()
        .filter(|entry| entry.1.links.is_some())
        .collect();
//    let keys : HashSet<&String> = HashSet::from_iter(connected_nodes.keys().cloned());
//    let values : HashSet<String> = HashSet::from_iter(connected_nodes.values().cloned().flat_map(|n: &Node| n.links.unwrap().into_iter()));
//    println!("{:?} {:?}", keys, values);
}