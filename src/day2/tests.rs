use utils::file::read_puzzle_input;
use day2::to_spreadsheet;
use day2::part1::solve;

#[test]
fn example() {
    let spready = to_spreadsheet("aoc2-example");
    assert_eq!(3, spready.len());
    assert_eq!(18, solve(spready));
}

#[test]
fn part1() {
    let spready = to_spreadsheet("aoc2");
    assert_eq!(32020, solve(spready));
}
