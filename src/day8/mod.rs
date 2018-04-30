use regex::Regex;
use regex::Match;
use std::collections::HashMap;
use std::ops::Add;
use std::ops::Sub;

#[cfg(test)]
pub mod tests;

#[derive(PartialEq)]
enum OPERATION {
    INC,
    DEC
}

struct Instruction{
    register : String,
    operation: OPERATION,
    value: i32,
    conditional_register :String,
    conditional_operation: String,
    conditional_value: i32
}

impl Instruction {
    
    fn meets_condition(&self, registers : &HashMap<String, i32>) -> bool {
        let value = *registers.get(&self.conditional_register).unwrap_or(&0);
        match self.conditional_operation.as_str() {
            ">" => value > self.conditional_value,
            ">=" => value >= self.conditional_value,
            "<" => value < self.conditional_value,
            "<=" => value <= self.conditional_value,
            "==" => value == self.conditional_value,
            "!=" => value != self.conditional_value,
            _  => panic!("unknown operator {}", self.conditional_operation)
        }
    }
    
    pub fn eval<'a>(&self, registers : &'a mut HashMap<String, i32>) -> &'a i32 {
        if self.meets_condition(registers) {
            let op = match self.operation {
                OPERATION::INC => i32::add,
                OPERATION::DEC => i32::sub
            };
            
            registers.entry(self.register.clone())
                    .and_modify(|e| *e = op(*e, self.value))
                    .or_insert(op(0,self.value));
            
        };
        registers.get(&self.register).unwrap_or(&0)
    }
}

struct CPU {
    registers : HashMap<String, i32>,
    instructions: Vec<Instruction>
}

impl CPU {
    
    pub fn eval(&mut self) -> Vec<i32> {
        let mut results = vec![];
        for i in &self.instructions {
            results.push(*i.eval(&mut self.registers));
        }
        results
    }
    
    pub fn get_max_value(&self) -> &i32 {
        self.registers.values().max().unwrap_or(&0)
    }
    
}

fn to_cpu(s: &String) -> CPU {
    CPU { 
        registers    : HashMap::new(), 
        instructions : s.lines().map(to_instruction).collect() 
    }
}

fn to_instruction(line: &str) -> Instruction {
    lazy_static! {
        static ref NODE_REGEX: Regex = Regex::new(r"^(\w+) (inc|dec) ([-0-9]+) if (\w+) ([<>=!]{1,2}) ([-0-9]+)$").unwrap();
    }
    let cap = NODE_REGEX.captures_iter(line).next().unwrap();

    let register = capture_group_as_string(cap.get(1));
    let operation = cap.get(2).map_or(OPERATION::INC, |m| match m.as_str(){
        "inc" => OPERATION::INC,
        _     => OPERATION::DEC
    });
    let value: i32 = capture_group_as_int(cap.get(3));
    let conditional_register = capture_group_as_string(cap.get(4));
    let conditional_operation = capture_group_as_string(cap.get(5));
    let conditional_value: i32 = capture_group_as_int(cap.get(6));
    
    Instruction {
        register,
        operation,
        value,
        conditional_register,
        conditional_operation,
        conditional_value
    }
}

fn capture_group_as_string(capture: Option<Match>) -> String {
    capture.map_or("NO VALUE FOUND".to_string(), |m| m.as_str().to_string())
}

fn capture_group_as_int(capture: Option<Match>) -> i32 {
    capture.map_or(0, |m| m.as_str().parse().unwrap_or(0))
}