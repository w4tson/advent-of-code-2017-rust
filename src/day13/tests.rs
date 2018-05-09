use utils::file::read_puzzle_input;
use std::collections::HashMap;
use day13::calc_severity;
use day13::is_scan_at_row_zero;

#[test]
fn part1() {
    let firewall_config = read_puzzle_input("aoc13")
        .lines()
        .map(to_entry)
        .collect::<HashMap<i32, i32>>();    
    
    assert_eq!(788, calc_severity(firewall_config));
}

fn to_entry(line: &str) -> (i32, i32) {
    let ints : Vec<i32> = line.split(": ").map(|a| a.parse().unwrap_or(-1)).collect();
    match &ints[..] {
        &[first, second, ref _tail..] => (first, second),
        _ => unreachable!(),
    } 
}

