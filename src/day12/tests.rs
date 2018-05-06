use utils::file::read_puzzle_input;
use day12::connections;
use day12::all_connected_to;

#[test]
fn part1() {
    let connections = connections(read_puzzle_input("aoc12"));
    assert_eq!(378, all_connected_to(0, connections).len());
    
}