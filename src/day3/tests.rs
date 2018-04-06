use day3::part1::calc_corner;
use day3::part1::ring;
use day3::part1::solve;

#[test]
fn part1() {
    assert_eq!(419, solve(289326));
}

#[test]
fn examples() {
    assert_eq!(0, solve(1));
    assert_eq!(3, solve(12));
    assert_eq!(2, solve(23));
    assert_eq!(31, solve(1024));
    
}

#[test  ]
fn test_bottom_corner() {
    assert_eq!(1, calc_corner(1));
    assert_eq!(9, calc_corner(2));
    assert_eq!(9, calc_corner(3));
    assert_eq!(9, calc_corner(4));
    assert_eq!(9, calc_corner(5));
    assert_eq!(9, calc_corner(6));
    assert_eq!(9, calc_corner(7));
    assert_eq!(9, calc_corner(8));
    assert_eq!(9, calc_corner(9));
    assert_eq!(25, calc_corner(10));
    assert_eq!(25, calc_corner(11));
    assert_eq!(25, calc_corner(12));
    assert_eq!(25, calc_corner(13));
    assert_eq!(25, calc_corner(14));
    assert_eq!(25, calc_corner(15));
    assert_eq!(25, calc_corner(16));
    assert_eq!(25, calc_corner(17));
    assert_eq!(25, calc_corner(18));
    assert_eq!(25, calc_corner(19));
    assert_eq!(25, calc_corner(20));
    assert_eq!(25, calc_corner(21));
    assert_eq!(25, calc_corner(22));
    assert_eq!(25, calc_corner(23));
    assert_eq!(25, calc_corner(24));
    assert_eq!(25, calc_corner(25));
    assert_eq!(49, calc_corner(26));
    assert_eq!(49, calc_corner(27));
    assert_eq!(49, calc_corner(28));
    assert_eq!(49, calc_corner(29));
    assert_eq!(49, calc_corner(30));
    assert_eq!(49, calc_corner(31));
    assert_eq!(49, calc_corner(31));
}

#[test]
fn test_ring() {
    assert_eq!(1, ring(1));
    assert_eq!(2, ring(2));
    assert_eq!(2, ring(3));
    assert_eq!(2, ring(6));
    assert_eq!(2, ring(9));
    assert_eq!(3, ring(10));
}