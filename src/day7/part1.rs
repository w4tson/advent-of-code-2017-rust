use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
use day7::tree::Tree;
use day7::tree::Node;

pub fn root_node(graph: &Tree) -> &String {
    let connected_nodes: HashMap<&String, &Node> = graph.tree.iter()
        .collect();

    let key_set: HashSet<&String> = HashSet::from_iter(connected_nodes.keys().cloned());

    let values : Vec<&String> = connected_nodes.values().cloned().flat_map(|n| unwrap_children(&n.children)).collect();
    let values_set : HashSet<&String> = HashSet::from_iter(values);

    let mut difference = key_set.difference(&values_set);
    difference.next().unwrap()
}


//turns a vec of owned Strings to a vec of refs to Strings
fn unwrap_children(children : &Vec<String>) -> Vec<&String> {
    children.iter().collect()
}
