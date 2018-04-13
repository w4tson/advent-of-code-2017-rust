use std::cell::Cell;

pub fn calc_num_steps<F>(input_str: String, f: F) -> u32
    where F : Fn(i32) -> i32 {
    let instructions: Vec<Cell<i32>> = input_str.lines()
        .filter_map(|line| line.trim().parse::<i32>().map(Cell::new).ok())
        .collect();

    let mut acc= 0;
    let mut offset : i32 = 0;

    while offset >= 0 && offset < instructions.len() as i32 {
        let cell = instructions.get(offset as usize).unwrap();
        let jump = cell.get();
        cell.set(f(jump));
        offset += jump;

//      println!("{:?} offset = {} acc = {}", instr, offset, acc);

        acc += 1;
    }
    acc
}

