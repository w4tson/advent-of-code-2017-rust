
#[cfg(test)]
pub mod tests;

#[derive(PartialEq, Clone, Debug)]
enum Mode {
    GroupStart,
    GarbageStart,
    GarbageEnd
}

#[derive(Clone, Debug)]
pub struct State {
    mode: Mode,
    depth: i32,
    escaping: bool,
    total_groups: i32,
    total_garbage: i32
}

impl State {
    pub fn next(&self, ch : char) -> State {
        let mut next = self.clone(); 
        if self.escaping {
            next.escaping = false;
        } else {
            match ch {
                '!' => next.escaping = true,
                '>' => next.mode = Mode::GarbageEnd,
                '<' if self.mode != Mode::GarbageStart => next.mode = Mode::GarbageStart,
                '{' if self.mode != Mode::GarbageStart => next.depth += 1,
                '}' if self.mode != Mode::GarbageStart => { next.total_groups += self.depth; next.depth -= 1},
                _   if self.mode == Mode::GarbageStart => next.total_garbage += 1,
                _ => ()
            }
        }
        next
    }
    
    pub fn new() -> State {
        State { 
            mode: Mode::GroupStart, 
            depth: 0, 
            escaping: false, 
            total_groups: 0, 
            total_garbage: 0 
        }
    }
}

pub fn count_garbage(input : &String) -> State {
    input.chars().fold(State::new(), |state, ch| state.next(ch))
}
