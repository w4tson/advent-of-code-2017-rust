use day10::knot_hash2;

#[cfg(test)]
pub mod tests;

fn calc_used_sqaures(input: String) -> usize {
    (0..128)
        .map(|i| knot_hash2(format!("{}-{}", input, i)))
        .map(|hash| row_count(&hash))
        .sum()
}

fn row_count(input : &String) -> usize {
    let row: String = input.chars()
        .map(|c| format!("{:04b}", u32::from_str_radix(&c.to_string(), 16).expect("WTF")))
        .collect();
    row.chars().filter(|c| *c == '1').count()
}