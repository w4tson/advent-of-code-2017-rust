use day10::knot_hash;


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
