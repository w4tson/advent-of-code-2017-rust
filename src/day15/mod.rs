use std::ops::{Generator, GeneratorState};


//Generator A starts with 516
//Generator B starts with 190
#[cfg(test)]
pub mod tests;

fn foo() {
    let mut generator = || {
        yield 1;
        return "foo"
    };

    match generator.resume() {
        GeneratorState::Yielded(1) => {}
        _ => panic!("unexpected return from resume"),
    }

    match generator.resume() {
        GeneratorState::Complete("foo") => {}
        _ => panic!("unexpected return from resume"),
    }
    
}