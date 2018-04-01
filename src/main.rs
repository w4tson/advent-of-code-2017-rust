use std::str::Chars;

trait CircularIterable {
    fn circular_iter(&self) -> CircularIter;
}

struct CircularIter<'a> {
    iter : Chars<'a>,
    first : Option<char>
}

impl<'a> CircularIter<'a> {

    fn new(s: &'a str) -> CircularIter<'a> {
        CircularIter { iter: s.chars(), first: s.chars().nth(0) }
    }
    
}

impl<'a> Iterator for CircularIter<'a> {
    type Item = char;
    
    fn next(&mut self) -> Option<char> {

        match self.iter.next() {
            None => match self.first {
                None => None,
                Some(first) => {
                    self.first = None;
                    Some(first) 
                }
            },
            Some(x) => Some(x)
        }
    }
}

impl CircularIterable for str {
    
    fn circular_iter(&self) -> CircularIter {
        CircularIter::new(&self)
    }
}

fn main() {
    
    for  i in "abcdefghijkl".circular_iter() {
        println!("{}", i);
    }
    
}
