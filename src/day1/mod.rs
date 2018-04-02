
use super::utils::iter::CircularIterable;

pub fn day1(input : &str) -> u32 {
    let (acc, _) : (u32,u32) = input.circular_iter()
        .map(to_uint)
        .fold((0,0), | (acc, prev): (u32, u32), item| {
            match item == prev {
                true => (acc+item, item),
                _    => (acc, item)
            }
        });
    
    acc
    
}

fn to_uint(s : char) -> u32 {
    s.to_digit(10).unwrap()
}

#[cfg(test)]
mod tests {
    
    use super::*;
    use utils::file::read_puzzle_input;

    #[test]
    fn examples() {
        assert_eq!(3, day1("1122"));
        assert_eq!(4, day1("1111"));
        assert_eq!(0, day1("1234"));
        assert_eq!(9, day1("91212129"));
    }
    
    #[test]
    fn part1() {
        let input = read_puzzle_input("aoc1");
        let answer = day1(&input);
        println!("{}", answer);
    }
}