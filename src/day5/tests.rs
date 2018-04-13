use utils::file::read_puzzle_input;
use day5::part1::calc_num_steps;

fn inc(i : i32) -> i32 {
    i + 1
}

fn inc_part2(i: i32) -> i32 {
    match i >= 3 {
        true => i + -1,
        _    => i + 1
    }
}

#[test]
fn example() {
    assert_eq!(5, calc_num_steps(read_puzzle_input("aoc5-example"), inc));
}

#[test] 
fn part1() {
    assert_eq!(342669, calc_num_steps(read_puzzle_input("aoc5"), inc));
}

#[test]
fn example2() {
    assert_eq!(10, calc_num_steps(read_puzzle_input("aoc5-example"), inc_part2));
}

#[test]
fn part2() {
    assert_eq!(25136209, calc_num_steps(read_puzzle_input("aoc5"), inc_part2));
}
