use std::str::Chars;


pub trait CircularIterable {
    fn circular_iter(&self) -> CircularIter;
}

pub struct CircularIter<'a> {
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

pub trait CircularSubSlice {
    fn rev_circular_sub_slice(&mut self, pos: usize, length: usize);
}

impl CircularSubSlice for [i32] {

    fn rev_circular_sub_slice(&mut self, pos: usize, length: usize) {
        let mut sub = self.iter().cloned().cycle().skip(pos).take(length).collect::<Vec<i32>>();
        sub.reverse();
        for i in 0..length {
            let circular_index = (i + pos) % self.len();
            self[circular_index..circular_index+1].swap_with_slice(&mut sub[i..i+1]);
        }
    }
}

