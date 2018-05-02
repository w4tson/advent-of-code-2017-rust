use day10::knot_hash;
use day10::knot_hash2;


#[test]
fn example() {
    let input_lengths = vec![3, 4, 1, 5];
    assert_eq!(12, knot_hash(5, &input_lengths));
}

#[test]
fn part1() {
    let input_lengths = vec![227, 169, 3, 166, 246, 201, 0, 47, 1, 255, 2, 254, 96, 3, 97, 144];
    assert_eq!(13760, knot_hash(256, &input_lengths));
}

#[test]
fn part2() {
    assert_eq!("2da93395f1a6bb3472203252e3b17fe5", knot_hash2("227,169,3,166,246,201,0,47,1,255,2,254,96,3,97,144"));
}
