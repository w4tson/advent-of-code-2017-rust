use day2::Spreadsheet;


pub fn solve_part2(spreadsheet: &Spreadsheet) -> u32 {
    spreadsheet.iter().map(row_checksum).sum()
}

fn row_checksum(row: &Vec<u32>) -> u32 {
    match row.len() {
        0 | 1 => 0,
        _ => sum_list(&row) + row_checksum(&Vec::from(row.split_at(1).1))
    }
}

fn sum_list(arr: &Vec<u32>) -> u32 {
    let (head, tail) = arr.split_at(1);
    pair_with_list(head[0], &Vec::from(tail))
        .iter()
        .map(evenly_divisible)
        .sum()
} 

fn pair_with_list(item: u32, list: &Vec<u32>) -> Vec<(u32, u32)> {
    list.iter().map(|&i| (item, i)).collect()
}

fn evenly_divisible(pair: &(u32, u32)) -> u32 {
    match (pair.0, pair.1) {
        (x, y) if x % y == 0 => x / y,
        (x, y) if y % x == 0 => y / x,
        _ => 0
    }
}
