use utils::file::read_puzzle_input;
use std::collections::HashMap;
use day13::calc_severity;
use day13::calc_delay_to_avoid_getting_caught;

#[test]
fn part1() {
    assert_eq!(788, calc_severity(&build_firewall()));
}

#[test]
fn part2() {
    assert_eq!(3905748, calc_delay_to_avoid_getting_caught(&build_firewall()));
}

fn build_firewall() -> HashMap<i32, i32> {
    read_puzzle_input("aoc13")
        .lines()
        .map(to_entry)
        .collect::<HashMap<i32, i32>>()
}

fn to_entry(line: &str) -> (i32, i32) {
    let ints : Vec<i32> = line.split(": ").map(|a| a.parse().unwrap_or(-1)).collect();
    match &ints[..] {
        &[first, second, ref _tail..] => (first, second),
        _ => unreachable!(),
    } 
}

