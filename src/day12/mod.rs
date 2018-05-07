use std::collections::HashMap;

#[cfg(test)]
pub mod tests;

fn connections(input : String) -> HashMap<i32, Vec<i32>>{
    input.lines().map(to_entry).collect()
}

fn to_entry(line: &str) -> (i32, Vec<i32>) {
    let from_to : Vec<&str> = line.split(" <-> ").collect();

    let from = from_to[0].parse().unwrap_or(0);
    let to : Vec<i32> = from_to[1].split(", ").map(|i| i.parse().unwrap_or(0)).collect();
    
    (from , to)
}

fn all_connected_to(from: i32, connections: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut unvisited : Vec<i32>= vec![from];
    let mut result : Vec<i32> = vec![];
    
    
    while let Some(next) = unvisited.pop() {
        result.push(next);
        connections.get(&next).unwrap_or(&vec![]).iter()
            .filter(|&item| !result.contains(item))
            .for_each(|item| unvisited.push(*item));
    }
    
    result
}

fn find_groups(connections: HashMap<i32, Vec<i32>>) -> usize {
    find_groups_internal(connections, vec![]).len() as usize
}

fn find_groups_internal(connections: HashMap<i32, Vec<i32>>, groups: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    match connections.keys().last() {
        None => groups,
        Some(key) => {
            let new_group = all_connected_to(*key, &connections);
            let mut groups = groups;
            groups.push(new_group);
            let reduced_connections = 
                connections.clone().into_iter()
                .filter(|(k, _)|!groups.iter().flatten().any(|i| i == k))
                .collect();
            find_groups_internal(reduced_connections, groups)
            
        }
    }
}

