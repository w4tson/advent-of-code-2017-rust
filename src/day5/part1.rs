pub fn calc_num_steps(inputStr : String) -> u32 {
    let instr: Vec<i32> = inputStr.lines()
        .filter_map(|line| line.trim().parse::<i32>().ok())
        .collect();

    process(instr)
}

fn process(instructions : Vec<i32>) -> u32 {
    let mut acc= 0;
    let mut offset : i32 = 0;
    let len = instructions.len() as i32;
    let mut instr = instructions;
    while offset >= 0 && offset < len  {
        instr[offset as usize] += 1;
        {
            // unfortunately have to decrement here because of borrowing rules mean
            // the increment has happened first :/
            let jump = instr.get(offset as usize).unwrap() -1; 
            offset = offset + jump;
        }

//      println!("{:?} offset = {} acc = {}", instr, offset, acc);

        acc += 1;
    }
    acc
}