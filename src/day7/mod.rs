use std::collections::HashMap;
use regex::Regex;
use regex::Match;
use itertools::Itertools;


pub mod part1;
#[cfg(test)]
pub mod tests;

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub value: i32,
    pub links: Vec<String>
}

#[derive(Debug)]
pub struct Graph {
    pub graph: HashMap<String, Node>
}

impl Graph {
    
    pub fn is_balanced(&self, node: &String) -> bool {
        let weights = self.weights_of_links(node);
        match weights.len() {
            0|1 => true,
            n   => {
                let first = weights.first().unwrap();
                weights.iter().sum::<i32>() / n as i32 == *first
            } 
        }
    }
    
    pub fn weights_of_links(&self, node: &String) -> Vec<i32> {
        let node = self.to_node(node);
        node.links.iter().map(|link| self.weight(link)).collect()
    }
    
    pub fn weight(&self, node: &String) -> i32 {
        let node = self.to_node(node);
        
        node.value + node.links.iter().map(|link| self.weight(link)).sum::<i32>()
    }
    
    pub fn to_node(&self, node: &String) -> &Node {
        self.graph.get(node).unwrap()
    }
    
    pub fn links(&self, node: &String) -> Vec<&Node> {
        self.to_node(node).links.iter().map(|link| self.to_node(link)).collect()
    }
    
    pub fn children_grouped_by_weight(&self, node: &String) -> Vec<(i32, Vec<&Node>)> {
        self.links(node).iter()
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
}


fn build_graph(s: String) -> Graph {
    
    let g: HashMap<String, Node> = s.lines()
        .map(to_entry)
        .collect();
    
    Graph {
        graph: g
    }
        
}

fn to_entry(line: &str) -> (String, Node) {
    let node = to_node(line);
    (node.name.clone(), node)
}

fn to_node(line: &str) -> Node {
    lazy_static! {
        static ref NODE_REGEX: Regex = Regex::new(r"^(\w+)\s+\((\d+)\)(\s+->\s+(\w+)(,\s*\w+)*)?").unwrap();
    }
    
    let cap = NODE_REGEX.captures_iter(line).next().unwrap();
    
    let name = cap.get(1).map_or("".to_string(), |m| m.as_str().to_string());
    let value = cap.get(2).map_or(0, |m| m.as_str().parse().unwrap_or(0));
    let links: Vec<String> = cap.get(3).map(to_links).unwrap_or(vec![]);
    
//    println!("{} ({}) ->  {:?}",name, value,  links);
    
    Node {
        name,
        value,
        links
    }
}

fn to_links(links : Match) -> Vec<String> {
        links.as_str()
            .split_at(3)
            .1
            .trim()
            .split(", ")
            .map(String::from)
            .collect()
}



