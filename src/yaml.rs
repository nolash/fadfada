use std::str::FromStr;
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
        let endpoint_url = Url::parse(&url_string).unwrap();

        let mut endpoint_url_port: u16 = endpoint_url.port_or_known_default().unwrap();
        Endpoint::new(
            endpoint_url.scheme(),
            endpoint_url.host_str().unwrap(),
            &endpoint_url_port,
            Some(endpoint_url.path()),
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
            _ => {},
        };
        return source;
    }
}

impl FromStr for Controller {
    type Err = ScanError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = YamlLoader::load_from_str(s);
        match r {
            Ok(y) => {
                let v = y[0].as_hash().unwrap();
                let scheduler = Scheduler::from_yaml(&v, None);
                let ctrl = Controller::new(scheduler);
                return Ok(ctrl);
            },
            Err(e) => {
                return Err(e);
            }
        }
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
