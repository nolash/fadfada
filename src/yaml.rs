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

trait FromYaml<T> {
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
        debug!("endpoint {:?}", endpoint_url);

        let mut endpoint_url_port: u16;
        match endpoint_url.port() {
            Some(port) => {
                endpoint_url_port = port;
            },
            None => {
                endpoint_url_port = 0;
            },
        }
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

fn yaml_from_str(s: &str) -> Hash {
    let yaml_docs = YamlLoader::load_from_str(s).unwrap();
    let y = yaml_docs[0].as_hash().unwrap();
    y.clone()
}

#[cfg(test)]
mod tests {
    use super::Scheduler;
    use log::debug;
    use env_logger;
    use super::yaml_from_str;
    use super::FromYaml;

    #[test]
    fn test_yaml_scheduler() {
        env_logger::init();
        let mut s = "delay: 13 \n\
        timeout: 42 \n\
";

        let mut y = yaml_from_str(&s);
        let mut scheduler = Scheduler::from_yaml(&y, None);
        assert_eq!(scheduler.delay, 13);
        assert_eq!(scheduler.timeout, 42);

        s = "delay: 111\n";
        y = yaml_from_str(&s);
        let mut scheduler_overridden = Scheduler::from_yaml(&y, Some(&scheduler));
        assert_eq!(scheduler_overridden.delay, 111);
        assert_eq!(scheduler_overridden.timeout, 42);

        s = "timeout: 222\n";
        y = yaml_from_str(&s);
        scheduler_overridden = Scheduler::from_yaml(&y, Some(&scheduler));
        assert_eq!(scheduler_overridden.delay, 13);
        assert_eq!(scheduler_overridden.timeout, 222);

        s = "delay: 333 \n\
timeout: 444 \n\
";
        y = yaml_from_str(&s);
        scheduler_overridden = Scheduler::from_yaml(&y, Some(&scheduler));
        assert_eq!(scheduler_overridden.delay, 333);
        assert_eq!(scheduler_overridden.timeout, 444);
    }

    #[test]
    fn test_yaml_source() {
        use super::Source;
        use super::Yaml;

        let s = "sources:
\x20\x20- engine: foo
\x20\x20\x20\x20schedule:
\x20\x20\x20\x20\x20\x20delay: 22
\x20\x20\x20\x20\x20\x20timeout: 44
\x20\x20\x20\x20endpoints:
\x20\x20\x20\x20\x20\x20- url: http://foo.com
\x20\x20\x20\x20\x20\x20\x20\x20validator: foo
\x20\x20\x20\x20\x20\x20- url: https://bar.com/baz
\x20\x20\x20\x20\x20\x20\x20\x20validator: bar
";
        let y = yaml_from_str(&s);
        let mut k = Yaml::from_str("sources");
        let sources_y = y.get(&k).unwrap().as_vec().unwrap();
        let source_y = sources_y[0].as_hash().unwrap();
        let source = Source::from_yaml(&source_y, None);
        
        let source_timing = source.timing.unwrap();
        assert_eq!(source_timing.delay, 22);
        assert_eq!(source_timing.timeout, 44);
    }
}
