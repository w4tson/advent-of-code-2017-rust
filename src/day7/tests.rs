use utils::file::read_puzzle_input;
use day7::build_graph;
use day7::part1::root_node;
use day7::part2::amount_to_rebalance;

#[test]
fn part1() {
    let tree = build_graph(read_puzzle_input("aoc7"));
    assert_eq!("aapssr".to_string(), *root_node(&tree));
}

#[test]
fn part2() {
    let tree = build_graph(read_puzzle_input("aoc7"));
    assert_eq!(1458, amount_to_rebalance(tree));
}

