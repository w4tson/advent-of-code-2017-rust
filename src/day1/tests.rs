
use super::part1::solve;
use super::part2::solve as solve_part2;
use utils::file::read_puzzle_input;

#[test]
fn examples() {
    assert_eq!(3, solve("1122"));
    assert_eq!(4, solve("1111"));
    assert_eq!(0, solve("1234"));
    assert_eq!(9, solve("91212129"));
}

#[test]
fn part1() {
    let input = read_puzzle_input("aoc1");
    let answer = solve(&input);
    println!("{}", answer);
}

#[test]
fn part2_examples() {
    assert_eq!(6, solve_part2("1212"));
    assert_eq!(0, solve_part2("1221"));
    assert_eq!(4, solve_part2("123425"));
    assert_eq!(12, solve_part2("123123"));
    assert_eq!(4, solve_part2("12131415"));
}

#[test]
fn part2() {
    let input = read_puzzle_input("aoc1");
//    assert_eq!(950, solve_part2(&input));
}