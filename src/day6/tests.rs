use utils::file::read_puzzle_input;
use day6::part1::Memory;
use day6::part1::redistribute;
use std::cell::Cell;
use day6::part1::solve;
use day6::part1::index_of_max;

fn to_memory(s : String) -> Memory {
    s.split_whitespace()
        .filter_map(|i| i.parse::<u32>().map(Cell::new).ok())
        .collect()
}

#[test]
fn example() {
    println!("hello");
    let memory= to_memory(read_puzzle_input("aoc6-example"));
    let _redisributed = redistribute(&memory, 2);
    println!("redistributed = {:?}", _redisributed);
    
    assert_eq!(5, solve(&memory, 1));    
    
}

#[test]
fn test_max() {
    let memory= to_memory("1 2 2 1".to_string());
    assert_eq!(1, index_of_max(&memory));
}

#[test]
fn part1() {
    let memory= to_memory(read_puzzle_input("aoc6"));
    assert_eq!(14029, solve(&memory, 1));
}

#[test]
fn part2() {
    let memory= to_memory(read_puzzle_input("aoc6"));
    assert_eq!(2765, solve(&memory, 2) - 14029);
}