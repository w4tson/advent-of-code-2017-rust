use day14::calc_used_squares;
use day14::calc_groups;

#[test]
fn part1() {
    assert_eq!(8108, calc_used_squares("flqrgnkx".to_string()));
}

#[test]
fn part2() {
    calc_groups(&"flqrgnkx".to_string());  
}
