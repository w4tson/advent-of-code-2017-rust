use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;


#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub value: i32,
    pub links: Option<Vec<String>>
}

#[derive(Debug)]
pub struct Graph {
    pub graph: HashMap<String, Node>
}


pub fn root_node(graph: &Graph) -> &String {
    let connected_nodes: HashMap<&String, &Node> = graph.graph.iter()
        .collect();

    let key_set: HashSet<&String> = HashSet::from_iter(connected_nodes.keys().cloned());

    let values : Vec<&String> = connected_nodes.values().cloned().flat_map(|n| unwrap_links(&n.links)).collect();
    let values_set : HashSet<&String> = HashSet::from_iter(values);

    let mut difference = key_set.difference(&values_set);
    difference.next().unwrap()
}


fn unwrap_links(maybe_links : &Option<Vec<String>>) -> Vec<&String> {
    match maybe_links {
        None => vec![],
        Some(links) => links.iter().collect()
    }
}
