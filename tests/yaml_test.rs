use std::{
        path,
        fs,
    };

use log::{
    debug,
    info,
    warn,
    error,
};

use yaml_rust::Yaml;

use fadafada::timing::Scheduler;
use fadafada::source::Source;
use fadafada::endpoint::Endpoint;
use fadafada::control::Controller;
use fadafada::resolver::Resolver;
use fadafada::yaml::{
    yaml_from_str,
    FromYaml,
};

mod mock;
use mock::TestResolverItem;

#[test]
#[cfg(feature= "yaml")]
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
#[cfg(feature= "yaml")]
fn test_yaml_endpoint() {
    let yaml_src_path = path::Path::new(".")
        .join("testdata")
        .join("endpoint.yaml");

    let s = fs::read_to_string(&yaml_src_path).unwrap();
    let y = yaml_from_str(&s);
    let endpoint = Endpoint::from_yaml(&y, None);
    let resource = "deadbeef".to_string();
    assert_eq!(endpoint.url_for(&resource), "https://foo.com:443/deadbeef");
}

#[test]
#[cfg(feature= "yaml")]
fn test_yaml_source() {
    let yaml_src_path = path::Path::new(".")
        .join("testdata")
        .join("source.yaml");

    let s = fs::read_to_string(&yaml_src_path).unwrap();
    let y = yaml_from_str(&s);

    let mut k = Yaml::from_str("sources");
    let sources_y = y.get(&k).unwrap().as_vec().unwrap();
    let source_y = sources_y[0].as_hash().unwrap();
    let source = Source::from_yaml(&source_y, None);
    
    let source_timing = source.timing.unwrap();
    assert_eq!(source_timing.delay, 22);
    assert_eq!(source_timing.timeout, 44);
    assert_eq!(source.endpoints.len(), 2);
}

#[test]
#[cfg(feature = "yaml")]
fn test_yaml_controller() {

    env_logger::init();

    let yaml_src_path = path::Path::new(".")
        .join("testdata")
        .join("source.yaml");

    let s = fs::read_to_string(&yaml_src_path).unwrap();
    let y = yaml_from_str(&s);
  
    let mut ctrl = Controller::from_yaml(&y, None);

    let mut resolver = Resolver::new();
    let resolver_item_foo = TestResolverItem{
        key: vec![1, 2, 3],
    };
    resolver.add("foo".to_string(), &resolver_item_foo);
    let resolver_item_bar = TestResolverItem{
        key: vec![4, 5, 6],
    };
    resolver.add("bar".to_string(), &resolver_item_bar);

    let ctrl_graph = ctrl.generate(resolver);
    ctrl_graph.for_each(|v| {
        debug!("element {} {}", v.0, v.1);
        //assert_eq!(v.0, 13);
    });
}
