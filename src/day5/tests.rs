use utils::file::read_puzzle_input;
use day5::part1::calc_num_steps;

#[test]
fn example() {
    assert_eq!(5, calc_num_steps(read_puzzle_input("aoc5-example")));
}

#[test]
fn part1() {
    assert_eq!(342669, calc_num_steps(read_puzzle_input("aoc5")));
    
}