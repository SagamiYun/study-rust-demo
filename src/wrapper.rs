use std::fmt;

pub struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

impl Wrapper {
    pub fn push(&mut self, s: String) -> usize {
        self.0.push(s);
        self.0.len()
    }

    pub fn remove(&mut self, index: usize) -> String {
        self.0.remove(index)
    }

    pub fn get(&self, index: usize) -> Option<&String> {
        self.0.get(index)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}