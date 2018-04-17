use utils::file::read_puzzle_input;
use day7::build_graph;
use day7::part1::root_node;


#[test]
fn part1() {
    let graph = build_graph(read_puzzle_input("aoc7"));
    let root_node = root_node(&graph);
    assert_eq!("aapssr".to_string(), *root_node);
}

