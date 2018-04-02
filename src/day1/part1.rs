use super::super::utils::iter::CircularIterable;
use super::super::utils::to_uint;

pub fn solve(input : &str) -> u32 {
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
