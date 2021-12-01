use std::fmt;
use std::collections::HashMap;

pub struct ControllerGraph {
    v: HashMap<u64, String>,
    it: Vec<u64>,
    cursor: u16,
}

impl ControllerGraph {
    pub fn new() -> ControllerGraph {
        ControllerGraph{
            v: HashMap::new(),
            cursor: 0,
            it: Vec::<u64>::new(),
        }
    }
    pub fn add(&mut self, d: u64, e: String) {
        let mut r: bool = false;
        let mut offset: u64 = d * 1000;

        while r {
            match self.v.entry(offset) {
                x => {
                    offset += 1;
                },
                _ => {
                   r = false;
                },
            }
        }
        self.v.insert(offset, e);
    }

    pub fn get_by_offset(&self, d: u64) {
        
    }
}

impl Iterator for ControllerGraph {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        match self.it.len() {
            0 => {
                self.it = Vec::<u64>::new();
                for k in self.v.keys() {
                    self.it.push(*k);
                }
                self.it.sort();
            },
            _ => {},
        }
        let i: u64 = self.cursor as u64;
        match self.v.get(&i) {
            Some(x) => {
                self.cursor += 1;
                return Some(x.to_string());
            },
            None => {
                None
            },
        }
    }
}

impl fmt::Display for ControllerGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        // consider tradeoffs against BtreeMap; which is faster for a single sort?
        self.it.iter().for_each(|k| {
           write!(f, "{} {}\n", k, self.v.get(k).unwrap()); 
        });
        Ok(())
    }
}

