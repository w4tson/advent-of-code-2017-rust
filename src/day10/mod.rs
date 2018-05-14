use utils::iter::CircularSubSlice;

#[cfg(test)]
pub mod tests;


pub fn knot_hash2<T : Into<String>>(input_str: T) -> String {
    let input_lengths = input_str.into()
        .as_bytes()
        .iter()
        .chain([17, 31, 73, 47, 23].iter())
        .map(|i| *i as usize)
        .collect::<Vec<usize>>();

    let mut input: Vec<i32> = (0..256).collect();

    let mut pos : usize = 0;
    let mut skip : usize = 0;
    
    for _ in 0..64 {
        knot_hash_round(&mut input, 256, &input_lengths, &mut pos, &mut skip);
    }
    
    to_hex(&compute_dense_hash(&input))
}


pub fn knot_hash(input_size: i32, input_lengths: &Vec<usize>) -> i32 {
    let mut input: Vec<i32> = (0..input_size).collect();
    knot_hash_round(&mut input, input_size, input_lengths, &mut 0, &mut 0);
    input[0] * input[1]
}

fn knot_hash_round(input : &mut Vec<i32>, input_size: i32, input_lengths: &Vec<usize>, pos: &mut usize, skip: &mut usize) {

    for length in input_lengths {
        input.rev_circular_sub_slice(*pos, *length);
        *pos = ((*pos + length + *skip) as i32 % input_size) as usize;
        *skip += 1;
    }
}


fn to_hex(dense_hash: &Vec<i32>) -> String {
    dense_hash.iter().map(|num| format!("{:02x}", *num)).collect()
}

fn compute_dense_hash(sparse_hash: &Vec<i32>) -> Vec<i32> {
    sparse_hash.chunks(16).map(|chunk|  xor_all(chunk)).collect()
}

fn xor_all(arr: &[i32]) -> i32 {
    arr.iter().fold(0, |a, b| a ^ b)
}

