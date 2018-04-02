use utils::to_uint;

pub fn solve(input: &str) -> u32 {
    let half_size = input.len() / 2;
    let halfs_iter = input.chars().cycle().map(to_uint).skip(half_size).take(input.len());

    input.chars().map(to_uint).zip(halfs_iter)
        .fold(0, |acc, (a,b)| {
            match a == b {
                true => acc + a,
                _ => acc
            }
        })
}