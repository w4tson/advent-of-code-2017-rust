use utils::file::read_puzzle_input;
use day11::HexCoord;

#[test]
fn part1() {
    let input = read_puzzle_input("aoc11");
    
    let origin = HexCoord { x: 0, y: 0, z: 0 };
    
    let coord : HexCoord = input.split(",")
        .fold(origin.clone(), |acc, direction| acc.next_hex(direction));
    
    assert_eq!(796, coord.distance_from(&origin));
    
}