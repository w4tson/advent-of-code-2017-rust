use std::collections::HashMap;
use itertools::Itertools;

#[derive(Debug)]
pub struct Tree {
    pub tree: HashMap<String, Node>
}

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub value: i32,
    pub children: Vec<String>
}

impl Tree {

    pub fn weights_of_children(&self, node: &String) -> Vec<i32> {
        self.to_node(node).children.iter()
            .map(|child| self.weight(child))
            .collect()
    }

    pub fn children_grouped_by_weight(&self, node: &String) -> Vec<(i32, Vec<&Node>)> {
        self.child_nodes(node).iter()
            .map(|&node| (node, self.weight(&node.name)))
            .collect::<Vec<(&Node, i32)>>()
            .into_iter()
            .sorted_by_key(|item| item.1)
            .iter()
            .group_by(|item| item.1)
            .into_iter()
            .map(|(weight, group)| (weight, group.cloned().map(|(node, _)| node).collect::<Vec<&Node>>()))
            .collect()
    }

    pub fn get_unbalanced_child(&self, node: &String) -> Option<&Node> {
        self.children_grouped_by_weight(node)
            .iter()
            .filter(|(_, group)| group.len() == 1)
            .nth(0)
            .map(| (_, group)| group[0])
    }

    fn weight(&self, node: &String) -> i32 {
        let node = self.to_node(node);

        node.value + node.children.iter().map(|child| self.weight(child)).sum::<i32>()
    }

    fn to_node(&self, node: &String) -> &Node {
        self.tree.get(node).unwrap()
    }

    fn child_nodes(&self, node: &String) -> Vec<&Node> {
        self.to_node(node).children.iter().map(|child| self.to_node(child)).collect()
    }

}