use day10::knot_hash2;
use std::vec::Vec;
use std::collections::HashMap;
use day12::find_groups;

#[cfg(test)]
pub mod tests;

fn calc_used_squares(input: String) -> usize {
    (0..128)
        .map(|i| knot_hash2(format!("{}-{}", input, i)))
        .map(|hash| row_count(&hash))
        .sum()
}

fn calc_groups(input: &String) -> usize {
    //a grid modelled as flat vector
    //using -1 for empty cell and the product of its coords give a unique number for its value
    let grid : Vec<i32> = (0..128)
        .flat_map(|row_index| {
            to_row_string(&knot_hash2(format!("{}-{}", input, row_index)))
            .chars()
            .enumerate()
            .map(|(col_index, c)| match c {
                '1' => row_index*128 + col_index as i32,
                _   => -1
            }).collect::<Vec<i32>>()
            
            
    
    }).collect();

    println!("len = {}", grid.len());
    
    // by modelling as a tree of ints we can leverage Day12 part2 solution!
    let tree : HashMap<i32, Vec<i32>> = grid.iter()
        .filter(|&it| *it > 0 )
        .map(|it| (*it, adjacent_nodes(&grid, *it)))
        .collect();
    
    println!("{:?}",  grid);

    // Day 12 
    find_groups(tree)
}

fn adjacent_indicies(index: i32) -> Vec<i32> {
    let mut i : Vec<i32> = vec![];
    
    if index % 128 != 0 { i.push(index-1) }
    if index % 128 != 127 { i.push(index+1) }
    if index  > 127 { i.push(index-128) }
    if index < 127*128  { i.push(index+128) }
    
    return i
}

fn adjacent_nodes(data: &Vec<i32>, index: i32) -> Vec<i32> {
    match data[index as usize] {
        -1 => vec![],
        _ => adjacent_indicies(index).iter()
            .filter(|&it| data[*it as usize] != - 1 )
            .map(|a| *a).collect()
    }
}


fn to_row(row_string: &String) -> Vec<i32> {
    row_string.chars()
        .map(|c| match c {
            '1' => 1,
            _   => 0
        })
        .collect()
}

fn to_row_string(input: &String) -> String {
    input.chars()
        .map(|c| format!("{:04b}", i32::from_str_radix(&c.to_string(), 16).expect("WTF")))
        .collect()
}

fn row_count(input : &String) -> usize {
    to_row_string(&input).chars().filter(|c| *c == '1').count()
}