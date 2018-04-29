use regex::Regex;
use regex::Match;
use day7::tree::Tree;
use day7::tree::Node;

pub mod part1;
pub mod part2;
pub mod tree;
#[cfg(test)]
pub mod tests;

// Funcs for building up the Tree from the input file

fn build_graph(s: String) -> Tree {
    Tree {
        tree: s.lines().map(to_entry).collect()
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
    let children: Vec<String> = cap.get(3).map(to_children).unwrap_or(vec![]);
    
    Node {
        name,
        value,
        children
    }
}

fn to_children(children : Match) -> Vec<String> {
    children.as_str()
            .split_at(3)
            .1
            .trim()
            .split(", ")
            .map(String::from)
            .collect()
}



