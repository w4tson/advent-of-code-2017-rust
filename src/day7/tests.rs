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
    let mut path : Vec<String> = vec![]; 
    
    while node.is_some() {
        path.push(node.unwrap().clone());
        node = graph.get_unbalanaced_child(node.unwrap()).map(|r| &r.name);
        println!("next unbalanced child {:?}", node);
    }

    let last_unbalanced_node_parent = &path[path.len()-2]; //YOLO
    let last_unbalanced_node = &path[path.len()-1];
    
    println!("{:?}", graph.weights_of_links(&last_unbalanced_node));
    println!("{:?}", graph.weights_of_links(&last_unbalanced_node_parent));

    let sum_of_children_of_last_unbalanced: i32 = graph.weights_of_links(last_unbalanced_node).iter().sum();
    let expected_weight = 1486; //calculate this
    println!("{}", expected_weight - sum_of_children_of_last_unbalanced);
}

