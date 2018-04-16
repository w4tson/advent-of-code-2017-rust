use std::collections::HashMap;

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