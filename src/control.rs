use log::debug;

use crate::source::Source;
use crate::timing::Scheduler;
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
    pub sources: Vec<Source<'static>>,
    timing: Scheduler,
    pub offsets: Vec<u32>,
}

impl Controller {
    pub fn new(scheduler: Scheduler) -> Controller {
        Controller {
            sources: vec!(),
            offsets: vec!(),
            timing: scheduler,
        }
    }

    /// Add a source to the request collection.
    ///
    /// Sources will be requested in the order they were added.
    pub fn add(&mut self, source: Source<'static>) {
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
        debug!("controller added source {:?}", source);
        self.sources.push(source);
    }

    /// Generate a [ControllerGraph] from the current state of the [Controller].
    pub fn generate(&mut self, resolver: &Resolver) -> ControllerGraph {
        let mut g: ControllerGraph = ControllerGraph::new();
        self.sources.iter().enumerate().for_each(|(i, s)| {
            debug!("processing source {:?}", s);
            s.endpoints.iter().enumerate().for_each(|(j, e)| {
                let mut offset: u32 = self.offsets[i] as u32;
                match &s.timing {
                    Some(x) => {
                        let pointer = resolver.pointer_for(&s.engine).unwrap();
                        offset += x.delay * (j as u32);
                        let pointer_url = e.url_for(&pointer);
                        g.add(offset as u64, &s.engine, pointer_url); //.url_for(&pointer));
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
}
