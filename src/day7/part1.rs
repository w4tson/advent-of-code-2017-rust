use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
use day7::Graph;
use day7::Node;

pub fn root_node(graph: &Graph) -> &String {
    let connected_nodes: HashMap<&String, &Node> = graph.graph.iter()
        .collect();

    let key_set: HashSet<&String> = HashSet::from_iter(connected_nodes.keys().cloned());

    let values : Vec<&String> = connected_nodes.values().cloned().flat_map(|n| unwrap_links(&n.links)).collect();
    let values_set : HashSet<&String> = HashSet::from_iter(values);

    let mut difference = key_set.difference(&values_set);
    difference.next().unwrap()
}



fn unwrap_links(links : &Vec<String>) -> Vec<&String> {
    links.iter().collect()
}
