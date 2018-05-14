use day14::calc_used_squares;
use day14::calc_groups;
use day14::adjacent_indicies;
use spectral::assert_that;
use spectral::prelude::*;


#[test]
fn part1() {
    assert_eq!(8108, calc_used_squares("flqrgnkx".to_string()));
}

#[test]
fn part2() {
    let num_of_groups = calc_groups(&"stpzcrnm".to_string());
    println!("{}", num_of_groups);
    assert_eq!(1113, num_of_groups)
}

#[test]
fn test_adjacent_indicies() {
    assert_eq!(vec![1, 128], adjacent_indicies(0));
    assert_that(&adjacent_indicies(129)).contains_all_of(&vec![&1, &128, &130, &(129+128)])
}