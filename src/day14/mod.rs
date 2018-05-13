use day10::knot_hash2;
use std::vec::Vec;

#[cfg(test)]
pub mod tests;

fn calc_used_squares(input: String) -> usize {
    (0..128)
        .map(|i| knot_hash2(format!("{}-{}", input, i)))
        .map(|hash| row_count(&hash))
        .sum()
}

fn calc_groups(input: &String) -> usize {
    let grid : Vec<usize> = (0..128)
        .flat_map(|row_index| {
        knot_hash2(format!("{}-{}", input, row_index)).chars()
            .enumerate()
            .map(|(col_index, c)| match c {
                '1' => row_index*128 + col_index,
                _   => 0
            }).collect::<Vec<usize>>()
    
    }).collect();
    
    println!("{:?}",  grid);
    
    0
}



fn to_row(row_string: &String) -> Vec<u32> {
    row_string.chars()
        .map(|c| match c {
            '1' => 1,
            _   => 0
        })
        .collect()
}

fn to_row_string(input: &String) -> String {
    input.chars()
        .map(|c| format!("{:04b}", u32::from_str_radix(&c.to_string(), 16).expect("WTF")))
        .collect()
}

fn row_count(input : &String) -> usize {
    to_row_string(&input).chars().filter(|c| *c == '1').count()
}