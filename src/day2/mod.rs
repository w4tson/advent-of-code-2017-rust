use utils::file::read_puzzle_input;

pub mod part1;
pub mod part2;
#[cfg(test)]
pub mod tests;

pub type Spreadsheet = Vec<Vec<u32>>;

pub fn to_spreadsheet(input : &str) -> Spreadsheet {
    read_puzzle_input(input)
        .lines()
        .map(to_row)
        .collect()
}

pub fn to_row(row_string : &str) -> Vec<u32> {
    row_string.split_whitespace()
        .map(|s| s.parse::<u32>().unwrap_or(0))
        .collect()
}