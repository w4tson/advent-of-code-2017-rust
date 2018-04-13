use day4::part1::is_passphrase_valid;
use utils::file::read_puzzle_input;
use spectral::assert_that;
use spectral::prelude::*;

#[test]
fn examples() {
    assert_that(&is_passphrase_valid("aa bb cc dd ee".to_string())).is_some();
    assert_that(&is_passphrase_valid("aa bb cc dd aa".to_string())).is_none();
    assert_that(&is_passphrase_valid("aa bb cc dd aaa".to_string())).is_some();
}

#[test]
fn part1() {
    let valid_passphrases = read_puzzle_input("aoc4")
        .lines()
        .filter_map(is_passphrase_valid)
        .count();
    
    assert_eq!(466, valid_passphrases);
}