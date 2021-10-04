use std::fmt;
use std::collections::HashMap;
use itertools::Itertools;

use super::source::Source;
use super::timing::Scheduler;
use super::endpoint::Endpoint;

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
            it: Vec<u64>::new(),
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
    type Item = u64;

    fn next(&self) -> Option<u64> {
        match self.it {
            None => {
                self.it = self.v.keys();
                self.it = self.it.sorted();
            }
            _ => {},
        }
        match self.v.get(self.cursor) {
            Some(x) => {
                self.cursor += 1;
                return x as i64;
            },
            None => {
                Some(-1)
            },
        }
    }
}

impl fmt::Display for ControllerGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        // consider tradeoffs against BtreeMap; which is faster for a single sort?
        self.it.for_each(|k| {
           write!(f, "{} {}\n", k, self.v.get(k).unwrap()); 
        });
        Ok(())
    }
}

pub struct Controller {
    sources: Vec<Source<'static>>,
    timing: Scheduler,
    offsets: Vec<u32>,
}

impl Controller {
    fn new(scheduler: Scheduler) -> Controller {
        Controller {
            sources: vec!(),
            offsets: vec!(),
            timing: scheduler,
        }
    }
    fn add(&mut self, source: Source<'static>) {
        match self.timing.delay {
            x if x > 0 => match self.offsets.len() {
                0 => {
                    self.offsets.push(0);
                },
                y => {
                    let u: u32 = self.offsets[y-1] + x;
                    self.offsets.push(u);
                }
            },
            _ => {
                self.offsets.push(0);
            },
        }
        self.sources.push(source);
    }

    fn generate(&mut self, pointer: &String) -> ControllerGraph {
        let mut g: ControllerGraph = ControllerGraph::new();
        self.sources.iter().enumerate().for_each(|(i, s)| {
            s.endpoints.iter().enumerate().for_each(|(j, e)| {
                let mut offset: u32 = self.offsets[i] as u32;
                match &s.timing {
                    Some(x) => {
                        offset += x.delay * (j as u32);
                        g.add(offset as u64, e.url_for(pointer));
                    },
                    None => {},
                }
               // write!(f, "{} {} {} {}\n", i, j, offset, e);
            });
        });
        g 
    }
}

//impl fmt::Display for Controller {
//    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
//
//    }
//}

#[cfg(test)]
mod tests {
    use super::{
        Source,
        Controller,
        Scheduler,
        ControllerGraph,
    };
    use crate::endpoint::Endpoint;
    
    #[test]
    fn create() {
        let p: u16 = 443;
        let ea: Endpoint = Endpoint::new("https", "foo.com", &p, None, None);
        let eb: Endpoint = Endpoint::new("https", "bar.com", &p, Some("baz"), None);
        let ua: Scheduler = Scheduler {
            delay: 10,
            timeout: 2000,
        };
        let ub: Scheduler = Scheduler {
            delay: 20,
            timeout: 2000,
        };
        let sa: Source = Source{
            trusted_keys: vec!(),
            endpoints: vec!(ea, eb),
            //timing: None,
            timing: Some(ua),
        };
        let ec: Endpoint = Endpoint::new("http", "xyzzy.com", &p, None, None);
        let sb: Source = Source{
            trusted_keys: vec!(),
            endpoints: vec!(ec),
            //timing: None,
            timing: Some(ub),
        };
        let u: Scheduler = Scheduler {
            delay: 300,
            timeout: 2000,
        };
        let mut c: Controller = Controller::new(u);
        c.add(sa);
        c.add(sb);
      
        let ptr: String = "foo".to_string();
        let g: ControllerGraph = c.generate(&ptr);
        println!("{}", g);
    }
}