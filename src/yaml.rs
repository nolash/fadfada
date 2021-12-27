use log::{
    debug,
    info,
    warn,
    error,
};

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
use yaml_rust::scanner::ScanError;

use url::Url;

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
    fn from_yaml(y: &Hash, schedule_default: Option<&Scheduler>) -> Endpoint<'a> {
        let mut k = Yaml::from_str("url");
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

        let mut k = Yaml::from_str("sources");
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

impl<'a> FromYaml<Resolver<'a>> for Resolver<'a> {
    fn from_yaml(y: &Hash, schedule_default: Option<&Scheduler>) -> Resolver<'a> {
        let mut resolver = Resolver::new();
        //let mut initial = false;
        let mut items: Vec<(String, String)> = vec![];
        //let mut first_item: String = "".to_string();
        y.iter().for_each(|o| {
            //if !initial {
                //first_item = o.0.as_str().unwrap().to_string(); 
                //initial = true;
            //}
            let k = o.0.as_str().unwrap();
            let v = o.1.as_str().unwrap();
            items.push((k.to_string(), v.to_string()));
        });

        for item in items {
            let resolver_item = &SimpleResolverItem::new(item.1);
            resolver.add(item.0, resolver_item);
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
    use log::debug;
    use env_logger;
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
        let mut k = Yaml::from_str("bar");

        let r = y.get(&k).unwrap().as_vec().unwrap();
        assert_eq!(r.len(), 2);
    }
}
