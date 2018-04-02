extern crate aoc2017;

use aoc2017::utils::iter::CircularIterable;
use aoc2017::day1::foo;

fn main() {
    
    for  i in "abcdefghijkl".circular_iter() {
        println!("{}", i);
    }
    
    foo();
    
    
}
