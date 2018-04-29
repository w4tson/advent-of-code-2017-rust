use utils::file::read_puzzle_input;
use day7::build_graph;
use day7::part1::root_node;

#[test]
fn part1() {
    let graph = build_graph(read_puzzle_input("aoc7"));
    let root_node = root_node(&graph);
    assert_eq!("aapssr".to_string(), *root_node);
}

#[test]
fn part2() {
    let graph = build_graph(read_puzzle_input("aoc7"));
    let mut node = Some(root_node(&graph));
    let mut path : Vec<String> = vec![]; //route through the tree to the bad node
    
    while node.is_some() {
        path.push(node.unwrap().clone());
        node = graph.get_unbalanced_child(node.unwrap()).map(|r| &r.name);
        println!("next unbalanced child {:?}", node);
    }

    let last_unbalanced_node_parent = &path[path.len()-2]; //YOLO
    let last_unbalanced_node = &path[path.len()-1];
    
    println!("{:?}", graph.weights_of_links(&last_unbalanced_node));
    println!("{:?}", graph.weights_of_links(&last_unbalanced_node_parent));

    let sum_of_children_of_last_unbalanced: i32 = graph.weights_of_links(last_unbalanced_node).iter().sum();
    
    let children_by_weight = graph.children_grouped_by_weight(last_unbalanced_node_parent);
    let expected_weight = children_by_weight
        .iter()
        .filter(|(_, group)| group.len() > 1)
        .nth(0)
        .map(| (weight, _)| weight)
        .unwrap();
    
    assert_eq!(1458, expected_weight - sum_of_children_of_last_unbalanced);
}

