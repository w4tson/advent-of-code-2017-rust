use utils::iter::CircularSubSlice;

#[cfg(test)]
pub mod tests;


pub fn knot_hash2<T : Into<String>>(input_str: T) -> String {
    let input_lengths_ascii = input_str.into().chars().map(|c| (c as u8) as usize).collect::<Vec<usize>>();
    let suffix = vec![17, 31, 73, 47, 23];
    let input_lengths = [input_lengths_ascii, suffix].concat();

    let mut input: Vec<i32> = (0..256).collect();

    let mut pos : usize = 0;
    let mut skip : usize = 0;
    
    for _ in 0..64 {
        let (new_pos, new_skip) = knot_hash_round(&mut input, 256, &input_lengths, pos, skip);
        pos = new_pos;
        skip = new_skip;
    }
    
    to_hex(&compute_dense_hash(&input))
}


pub fn knot_hash(input_size: i32, input_lengths: &Vec<usize>) -> i32 {
    let mut input: Vec<i32> = (0..input_size).collect();
    let (pos, skip) = knot_hash_round(&mut input, input_size, input_lengths, 0, 0);
    input[0] * input[1]
}

fn knot_hash_round(input : &mut Vec<i32>, input_size: i32, input_lengths: &Vec<usize>, pos: usize, skip: usize) -> (usize, usize) {
    let mut pos: usize = pos;
    let mut skip: usize = skip;

    for length in input_lengths {
        input.rev_circular_sub_slice(pos, *length);
        pos = ((pos + length + skip) as i32 % input_size) as usize;
        skip += 1;
    }
    (pos, skip)
}


fn to_hex(dense_hash: &Vec<i32>) -> String {
    dense_hash.iter().map(|num| format!("{:x}", *num)).collect()
}

fn compute_dense_hash(sparse_hash: &Vec<i32>) -> Vec<i32> {
    sparse_hash.chunks(16).map(|chunk|  xor_all(chunk)).collect()
}

fn xor_all(arr: &[i32]) -> i32 {
    arr.iter().fold(0, |a, b| a ^ b)
}

