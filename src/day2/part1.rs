use day2::Spreadsheet;

pub fn solve(input : Spreadsheet) -> u32 {
    input.iter()
        .map(row_checksum)
        .sum()
}

fn row_checksum(row: &Vec<u32>) -> u32 {
    let mut min = <u32>::max_value();
    let mut max = 0;
    for i in row {
        if i > &max { max = *i; }
        if i < &min { min = *i; }
    }
    max - min
}