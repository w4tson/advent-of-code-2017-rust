use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::HashMap;

pub type Memory =  Vec<Cell<u32>>;

pub fn solve(memory : &Memory, c : u32) -> u32 {

    let mut seen_variations : HashMap<Vec<u32>, u32> = HashMap::new();

    for i in 1u32.. {
        
        let new_variation: Memory = redistribute(&memory, index_of_max(&memory)).clone();
        
        let nv: Vec<u32> = new_variation.into_iter().map(|i| i.get()).collect();
        
        let new_count = match seen_variations.get(&nv) {
           None => 1, 
           Some(x) => x + 1
        };

        seen_variations.insert(nv, new_count);
        
        if new_count > c { return i  };
        
    }
    

    0
}

pub fn redistribute(memory : &Memory, index_to_distribute: usize) -> &Memory {
    let cell = &memory[index_to_distribute];
    let to_distribute = cell.get() as usize;
    cell.set(0);
    let r: Vec<usize> = (0..memory.len()).collect();
    
    for index in r.iter().cycle().skip(index_to_distribute + 1).take(to_distribute) {
        let c = &memory[*index];
        c.set(c.get() + 1);
    };
    
    memory
}

pub fn index_of_max(memory: &Memory) -> usize {
    memory.iter()
        .enumerate()
        .max_by(|a, b| {
            match a.1.cmp(&b.1) {
                Ordering::Equal => b.0.cmp(&a.0),
                _a => _a
            }
        }).unwrap().0
}