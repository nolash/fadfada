use std::str::FromStr;
use log::{
    debug,
    info,
    warn,
    error,
};

use crate::control::Controller;
use crate::timing::Scheduler;

use yaml_rust::{
    Yaml,
    YamlLoader,
};
use yaml_rust::yaml::{
    Hash,
};
use yaml_rust::scanner::ScanError;


impl Scheduler {
    pub fn from_yaml(y: &Hash) -> Scheduler {
        let mut k = Yaml::from_str("delay");
        let offset = y.get(&k).unwrap().as_i64().unwrap();
        k = Yaml::from_str("timeout");
        let timeout = y.get(&k).unwrap().as_i64().unwrap();
        Scheduler {
            timeout: timeout as u32,
            delay: offset as u32,
        }
    }
}

impl FromStr for Controller {
    type Err = ScanError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = YamlLoader::load_from_str(s);
        match r {
            Ok(y) => {
                let v = y[0].as_hash().unwrap();
                let scheduler = Scheduler::from_yaml(&v);
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

    use std::str::FromStr;
    use super::Scheduler;
    use log::debug;
    use env_logger;
    use super::yaml_from_str;

    #[test]
    fn test_scheduler() {
        env_logger::init();
        let s = "delay: 13 \n\
        timeout: 42 \n\
";

        let y = yaml_from_str(&s);
        let scheduler = Scheduler::from_yaml(&y);
        assert_eq!(scheduler.delay, 13);
        assert_eq!(scheduler.timeout, 42);
    }
}
