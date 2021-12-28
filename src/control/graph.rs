use std::fmt;
use std::collections::HashMap;

use log::debug;

/// Represents the sequence and timings of a single resource request as described by the
/// [super:control.Controller] state at the time of request.
pub struct ControllerGraph {
    v: HashMap<u64, String>,
    it: Vec<u64>,
    it_active: bool,
}

impl ControllerGraph {
    pub fn new() -> ControllerGraph {
        ControllerGraph{
            v: HashMap::new(),
            it: Vec::<u64>::new(),
            it_active: false,
        }
    }

    /// Add a new offset/url pair to the graph.
    pub fn add(&mut self, d: u64, e: String) {
        let offset = self.find_next_offset(d);
       
        debug!("using offset {} (requested {}) for {}", offset, d, e);
        self.v.insert(offset, e);
    }

    fn find_next_offset(&self, offset_default: u64) -> u64 {
        let mut offset = offset_default;
        loop {
            match self.v.get(&offset) {
                Some(_) => {
                    offset += 1;
                },
                None => {
                    return offset;
                }
            }
        }
    }
}

impl Iterator for ControllerGraph {
    type Item = (u64, String);

    fn next(&mut self) -> Option<(u64, String)> {
        if !self.it_active {
            self.it = Vec::<u64>::new();
            for k in self.v.keys() {
                self.it.push(*k);
            }
            self.it.sort();
            self.it.reverse();
            self.it_active = true;
        }
        match self.it.pop() {
            Some(i) => {
                let s = self.v.get(&i).unwrap();
                Some((i, s.clone()))
            },
            None => {
                self.it_active = false;
                None
            },
        }
    }
}

impl fmt::Display for ControllerGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { //Result<(), fmt::Error> {
        // consider tradeoffs against BtreeMap; which is faster for a single sort?
        self.it.iter().for_each(|v| {
               match Some(v) {
                   Some(k) => {
                       let _r = fmt::write(f, format_args!("{} {:?}\n", k, self.v.get(k))); 
                   },
                   None => {},
               }
        });
        Ok(())
    }
}
