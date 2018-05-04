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

#[test]
fn part2() {
    let input = read_puzzle_input("aoc11");

    let origin = HexCoord { x: 0, y: 0, z: 0 };

    let max_distance : i32 = input.split(",")
        .fold(vec![origin.clone()], compute_distances)
        .iter()
        .map(|coord| coord.distance_from(&origin))
        .max()
        .unwrap_or(0);
    
    assert_eq!(1585, max_distance);
}

fn compute_distances(acc: Vec<HexCoord>, direction: &str) -> Vec<HexCoord> {
    let mut acc = acc;
    let next = acc.last().unwrap().next_hex(direction);
    acc.push(next);
    acc
}

