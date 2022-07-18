//! This optional module provides the convenience of defining contents and sources in yaml
//! documents, and automatically generating the
//! [ControllerGraph](crate::control::graph::ControllerGraph) from them.
//!
//! To generate a graph, two separate documents are required, the `control` document and the
//! `contents` document.
//!
//! # control
//!
//! This document specifies the scheduling and the network locations for each retrieval engine.
//!
//! To illustrate with an example:
//!
//! ``` ignore,
//! delay: 200
//! timeout: 4000
//! sources:
//!   - engine: foo
//!     endpoints:
//!       - url: http://one.foo.com
//!       - url: http://two.foo.com/foo
//!   - engine: bar
//!     endpoints:
//!       - url: http://only.bar.com
//! ```
//!
//! This results in requests for each engine fired 200 ms apart, where the request for the "bar"
//! engine is fired slightly after [^persched] the first request for the "foo" engine, resulting in something
//! like:
//! 
//! * after 0 ms: http://one.foo.com
//! * after 1 ms: http://only.bar.com
//! * after 200 ms: http://two.foo.com
//!
//! [^persched]: The ability to define per-engine offsets and schedules is intended but not yet implemented.
//!
//! # content
//!
//! The content file defines the reference for the content for every engine to be requested.
//!
//! The format is:
//!
//! ``` ignore,
//! foo: deadbeef
//! bar: beeffeed
//! ```
//!
//! Mapping this content with the engines defined in the control document from
//! the previous paragraph, the resulting query graph becomes:
//!
//! * after 0 ms: http://one.foo.com/deadbeef
//! * after 1 ms: http://only.bar.com/beeffeed
//! * after 200 ms: http://two.foo.com/deadbeef

use crate::control::Controller;
use crate::timing::Scheduler;
use crate::source::Source;
use crate::endpoint::Endpoint;
use crate::resolver::{
    Resolver,
    SimpleResolverItem,
};

use yaml_rust::{
    Yaml,
    YamlLoader,
};
use yaml_rust::yaml::{
    Hash,
};

pub trait FromYaml<T> {
    fn from_yaml(y: &Hash, schedule: Option<&Scheduler>) -> T;
}

impl FromYaml<Scheduler> for Scheduler {
    fn from_yaml(y: &Hash, schedule_defaults: Option<&Scheduler>) -> Scheduler {
        let mut schedule: Scheduler; 
        let mut k = Yaml::from_str("delay");
        let delay = y.get(&k); //.unwrap().as_i64().unwrap();
        k = Yaml::from_str("timeout");
        let timeout = y.get(&k); //.unwrap().as_i64().unwrap();

        match schedule_defaults {
            Some(v) => {
                schedule = (*v).clone();
            },
            None => {
                schedule = Scheduler {
                    timeout: 0,
                    delay: 0,
                };
            },
        };

        match delay {
            Some(v) => {
                schedule.delay = v.as_i64().unwrap() as u32;
            },
            _ => {},
        };

        match timeout {
            Some(v) => {
                schedule.timeout = v.as_i64().unwrap() as u32;
            },
            _ => {},
        };

        return schedule;
    }
}

impl<'a> FromYaml<Endpoint<'a>> for Endpoint<'a> {
    fn from_yaml(y: &Hash, _schedule_default: Option<&Scheduler>) -> Endpoint<'a> {
        let k = Yaml::from_str("url");
        let url_string = y.get(&k).unwrap().as_str().unwrap();
        Endpoint::new(
            url_string,
            None,
            )
    }
}

impl<'a> FromYaml<Source<'a>> for Source<'a> {
    fn from_yaml(y: &Hash, schedule_default: Option<&Scheduler>) -> Source<'a> {
        let mut k = Yaml::from_str("engine");
        let engine = y.get(&k).unwrap().as_str().unwrap();
        let mut source = Source::new(engine.to_string());
        k = Yaml::from_str("endpoints");
        let endpoints = y.get(&k).unwrap().as_vec().unwrap();
        for endpoint_entry in endpoints {
            let endpoint_y = endpoint_entry.as_hash().unwrap();
            let endpoint = Endpoint::from_yaml(endpoint_y, schedule_default);
            source.endpoints.push(endpoint);
        }

        k = Yaml::from_str("schedule");
        match y.get(&k) {
            Some(schedule_entry) => {
                let schedule_y = schedule_entry.as_hash().unwrap();
                source.timing = Some(Scheduler::from_yaml(schedule_y, None));
            }, 
            _ => {
                match schedule_default {
                    Some(v) => {
                        source.timing = Some(v.clone());
                    },
                    None => {
                        let scheduler_fallback = Scheduler {
                            delay: 0,
                            timeout: 0,
                        };
                        source.timing = Some(scheduler_fallback);
                    },
                };
            },
        };
        return source;
    }
}

impl FromYaml<Controller> for Controller {
    fn from_yaml(y: &Hash, schedule_default: Option<&Scheduler>) -> Controller {
        let schedule = Scheduler::from_yaml(y, schedule_default);

        let mut ctrl = Controller::new(schedule.clone()); //.clone());

        let k = Yaml::from_str("sources");
        match y.get(&k) {
            Some(sources_entry) => {
                for source_entry in sources_entry.as_vec().unwrap() {
                    let source_y = source_entry.as_hash().unwrap();
                    let source = Source::from_yaml(source_y, Some(&schedule));
                    ctrl.add(source);
                }
            },
            _ => {},
        };

        return ctrl;
    }
}

impl FromYaml<Resolver> for Resolver {
    fn from_yaml(y: &Hash, _schedule_default: Option<&Scheduler>) -> Resolver {
        let mut resolver = Resolver::new();
        let mut items: Vec<(String, String)> = vec![];
        y.iter().for_each(|o| {
            let k = o.0.as_str().unwrap();
            let v = o.1.as_str().unwrap();
            items.push((k.to_string(), v.to_string()));
        });

        for item in items {
            let resolver_item = SimpleResolverItem::new(item.1);
            let _r = resolver.add(item.0, Box::new(resolver_item));
        };

        resolver
    }
}

pub fn yaml_from_str(s: &str) -> Hash {
    let yaml_docs = YamlLoader::load_from_str(s).unwrap();
    let y = yaml_docs[0].as_hash().unwrap();
    y.clone()
}

#[cfg(test)]
mod tests {
    use super::yaml_from_str;
    use yaml_rust::{
        Yaml,
    };

    #[test]
    fn test_yaml_str() {
        let s = "foo: 42 \n\
bar: \n\
\x20\x20- one \n\
\x20\x20- two \n\
";
        let y = yaml_from_str(s);
        let k = Yaml::from_str("bar");

        let r = y.get(&k).unwrap().as_vec().unwrap();
        assert_eq!(r.len(), 2);
    }
}
