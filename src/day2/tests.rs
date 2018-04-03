use day2::to_spreadsheet;
use day2::part1::solve;
use day2::part2::solve_part2;

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

#[test]
fn part2_example() {
    assert_eq!(9, solve_part2(&to_spreadsheet("aoc2-example")));
}

#[test]
fn part2() {
    assert_eq!(236, solve_part2(&to_spreadsheet("aoc2")));
}
