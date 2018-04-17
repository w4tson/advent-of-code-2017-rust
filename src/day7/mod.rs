use day7::part1::Graph;
use day7::part1::Node;
use std::collections::HashMap;
use regex::Regex;
use regex::Match;

pub mod part1;
#[cfg(test)]
pub mod tests;

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
    let value= cap.get(2).map_or(0, |m| m.as_str().parse().unwrap_or(0));
    let links: Option<Vec<String>> = cap.get(3).map(to_links);
    
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



