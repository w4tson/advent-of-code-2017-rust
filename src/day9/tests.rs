use utils::file::read_puzzle_input;
use day9::count_garbage;

#[test]
fn part1and2() {
    let input = read_puzzle_input("aoc9");
    let state = count_garbage(&input);
    assert_eq!(23588, state.total_groups);
    assert_eq!(10045, state.total_garbage);
}

