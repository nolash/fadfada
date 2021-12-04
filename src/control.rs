use std::fmt;

use crate::source::Source;
use crate::timing::Scheduler;
use crate::endpoint::Endpoint;
use crate::resolver::Resolver;

pub mod graph;
use graph::ControllerGraph;

/// Controller defines the order and scheduling of how a collection of sources are
/// queried for content.
///
/// If a delay is defined in the schedule, the same delay will apply to every added source.
///
/// Each [Source] in turn defines its own delay applied to every [Endpoint] it contains.
///
/// Once a controller has been populated, it can be used to generate [ControllerGraph] instances,
/// which in turn can be used with a query engine to control the execution of a single query.
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

    /// Add a source to the request collection.
    ///
    /// Sources will be requested in the order they were added.
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

    /// Generate a [ControllerGraph] from the current state of the [Controller].
    //fn generate(&mut self, pointer: &String) -> ControllerGraph {
    fn generate(&mut self, resolver: Resolver) -> ControllerGraph {
        let mut g: ControllerGraph = ControllerGraph::new();
        self.sources.iter().enumerate().for_each(|(i, s)| {
            s.endpoints.iter().enumerate().for_each(|(j, e)| {
                let mut offset: u32 = self.offsets[i] as u32;
                match &s.timing {
                    Some(x) => {
                        let pointer = resolver.pointer_for(&s.engine).unwrap();
                        offset += x.delay * (j as u32);
                        g.add(offset as u64, e.url_for(&pointer));
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
    use crate::resolver::{
        Sha256ImmutableResolver,
        Resolver,
    };
    
    #[test]
    fn create_graph() {

        // port number
        let p: u16 = 443;

        // set up first source with endpoints and schedule
        let endpoint_a_one: Endpoint = Endpoint::new("https", "foo.com", &p, None, None);
        let endpoint_a_two: Endpoint = Endpoint::new("https", "bar.com", &p, Some("baz"), None);
        let sched_a: Scheduler = Scheduler {
            delay: 20,
            timeout: 2000,
        };
        let source_a: Source = Source{
            trusted_keys: vec!(),
            endpoints: vec!(endpoint_a_one, endpoint_a_two),
            timing: Some(sched_a),
            engine: "foo".to_string(),
        };

        // set up second source with endpoints and schedule
        let endpoint_b_one: Endpoint = Endpoint::new("http", "xyzzy.com", &p, None, None);
        let sched_b: Scheduler = Scheduler {
            delay: 10,
            timeout: 2000,
        };
        let source_b: Source = Source{
            trusted_keys: vec!(),
            endpoints: vec!(endpoint_b_one),
            timing: Some(sched_b),
            engine: "bar".to_string(),
        };

        // generate control graph from endpoints and schedules 
        let sched_ctrl: Scheduler = Scheduler {
            delay: 6,
            timeout: 2000,
        };
        let mut c: Controller = Controller::new(sched_ctrl);
        c.add(source_a);
        c.add(source_b);
      
        let mut resolver = Resolver::new();

        let key_one: Vec<u8> = vec![1, 2, 3];
        let ri_one: Sha256ImmutableResolver = Sha256ImmutableResolver{key: &key_one, content: None};
        resolver.add("foo".to_string(), &ri_one);

        let key_two: Vec<u8> = vec![4, 5, 6];
        let ri_two: Sha256ImmutableResolver = Sha256ImmutableResolver{key: &key_two, content: None};
        resolver.add("bar".to_string(), &ri_two);


        let g: ControllerGraph = c.generate(resolver);

        for v in g {
            println!(">> {:?}", v);
        }
    }
}
