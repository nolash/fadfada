extern crate fadafada;

use fadafada::source::Source;
use fadafada::control::Controller;
use fadafada::timing::Scheduler;
use fadafada::endpoint::Endpoint;
use fadafada::resolver::Resolver;

#[test]
#[cfg(feature = "web2")]
fn test_web2_create_graph() {

    use fadafada::web2::Sha256ImmutableResolverItem;

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
    let ri_one = Sha256ImmutableResolverItem{key: &key_one, content: None};
    resolver.add("foo".to_string(), &ri_one);

    let key_two: Vec<u8> = vec![4, 5, 6];
    let ri_two = Sha256ImmutableResolverItem{key: &key_two, content: None};
    resolver.add("bar".to_string(), &ri_two);


    let g = c.generate(resolver);

    for v in g {
        println!(">> {:?}", v);
    }
}
