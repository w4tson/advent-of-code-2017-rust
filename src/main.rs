extern crate aoc2017;

use aoc2017::utils::iter::CircularIterable;

fn main() {
    
    for  i in "abcdefghijkl".circular_iter() {
        println!("{}", i);
    }
    
}
