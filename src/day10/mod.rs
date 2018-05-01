
#[cfg(test)]
pub mod tests;

pub fn knot_hash(input_size: i32, input_lengths: &Vec<usize>) -> i32 {
    let mut input: Vec<i32> = (0..input_size).collect();
    let mut pos = 0;
    let mut skip = 0;
    
    for length in input_lengths {
        input.rev_circular_sub_slice(pos, *length);
        pos = ((pos + length + skip) as i32 % input_size) as usize;
        skip += 1;
    }
    
    input[0] * input[1]
}

trait KnotHash {
    fn rev_circular_sub_slice(&mut self, pos: usize, length: usize);
}

impl KnotHash for [i32] {
    
    fn rev_circular_sub_slice(&mut self, pos: usize, length: usize) {
        let ref mut sub = self.iter().cloned().cycle().skip(pos).take(length).collect::<Vec<i32>>();
        sub.reverse();
        for i in 0..length {
            let circular_index = (i + pos) % self.len();
            self[circular_index..circular_index+1].swap_with_slice(&mut sub[i..i+1]);
        }
    }
}
