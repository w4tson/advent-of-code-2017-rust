use utils::file::read_puzzle_input;
use day8::to_cpu;

#[test]
fn example() {
    let input = &read_puzzle_input("aoc8-example");
    let mut cpu = to_cpu(input);
    cpu.eval();
    assert_eq!(1, *cpu.get_max_value());
}

#[test]
fn part1() {
    let input = &read_puzzle_input("aoc8");
    let mut cpu = to_cpu(input);
    cpu.eval();
    assert_eq!(4888, *cpu.get_max_value());
}