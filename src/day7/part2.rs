use day7::part1::root_node;
use day7::tree::Tree;

pub fn amount_to_rebalance(graph: Tree) -> i32 {
    let mut node = Some(root_node(&graph));
    let mut path : Vec<String> = vec![]; //route through the tree to the bad node
    
    while let Some(next_node) = node {
        path.push(next_node.clone());
        node = graph.get_unbalanced_child(next_node).map(|r| &r.name);
        println!("next unbalanced child {:?}", node);
    }

    let last_unbalanced_node_parent = &path[path.len()-2]; //YOLO
    let last_unbalanced_node = &path[path.len()-1];

    println!("{:?}", graph.weights_of_children(&last_unbalanced_node));
    println!("{:?}", graph.weights_of_children(&last_unbalanced_node_parent));

    let sum_of_children_of_last_unbalanced: i32 = graph.weights_of_children(last_unbalanced_node).iter().sum();

    let children_by_weight = graph.children_grouped_by_weight(last_unbalanced_node_parent);
    let expected_weight = children_by_weight
        .iter()
        .filter(|(_, group)| group.len() > 1)
        .nth(0)
        .map(| (weight, _)| weight)
        .unwrap();
    
    expected_weight - sum_of_children_of_last_unbalanced
} 