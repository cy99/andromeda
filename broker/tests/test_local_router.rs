
use std::str::FromStr;
use std::net::SocketAddr;

use mqtt::{TopicFilter, TopicName, QualityOfService};

use broker::hub::local_router::{LocalRouteNode};


fn addr_from_port(port: &str) -> SocketAddr {
    SocketAddr::from_str(&format!("127.0.0.1:{}", port)).unwrap()
}

#[test]
fn test_local_route_node_simple() {
    // let default_user_id: u32 = 0;
    let mut routes = LocalRouteNode::new();
    let subs = vec![
        ("+", vec!["10"]),
        ("ab", vec!["20"]),
        ("ab/+", vec!["30", "31"]),
        ("ab/c", vec!["40"]),
        ("ab/d", vec!["50"]),
        ("xy/+", vec!["60", "61", "62"]),
        ("xy/+/t", vec!["62", "63"]),
        ("xy/#", vec!["70", "71"]),
        ("xy/xxxx", vec!["80"]),
    ];
    for &(topic_filter, ref ports) in subs.iter() {
        for port in ports {
            routes.insert(&TopicFilter::new(topic_filter),
                          addr_from_port(port), QualityOfService::Level0);
        }
    }

    let is_removed = routes.remove(&TopicFilter::new("Nothing/Will/Match"), addr_from_port("80"));
    assert_eq!(is_removed, false);
    let is_removed = routes.remove(&TopicFilter::new("ab"), addr_from_port("9999"));
    assert_eq!(is_removed, false);

    println!("[Routes]: {:#?}", routes);
    assert_eq!(routes.is_empty(), false);
    for &(topic_filter, ref ports) in subs.iter() {
        for port in ports {
            let is_removed = routes.remove(
                &TopicFilter::new(topic_filter), addr_from_port(port));
            assert_eq!(is_removed, true);
        }
    }
    println!("[Routes]: {:#?}", routes);
    assert_eq!(routes.is_empty(), true);
}

#[test]
fn test_local_route_node_normal() {
    // let default_user_id: u32 = 0;
    let mut routes = LocalRouteNode::new();
    let subs = vec![
        ("+", vec!["10"]),
        ("ab", vec!["20"]),
        ("ab/+", vec!["30", "31"]),
        ("ab/c", vec!["40"]),
        ("ab/d", vec!["50"]),
        ("xy/+", vec!["60", "61", "62"]),
        ("xy/+/t", vec!["62", "63"]),
        ("xy/#", vec!["70", "71"]),
        ("xy/xxxx", vec!["80"]),
    ];
    for (topic_filter, ports) in subs {
        for port in ports {
            routes.insert(&TopicFilter::new(topic_filter),
                          addr_from_port(port), QualityOfService::Level0);
        }
    }

    println!("[Routes]: {:#?}", routes);
    let cases = vec![
        ("x", vec!["10"]),
        ("ab", vec!["10", "20"]),
        ("ab/c", vec!["30", "31", "40"]),
        ("xy/a", vec!["60", "61", "62", "70", "71"]),
        ("xy/333/t", vec!["62", "63", "70", "71"]),
        ("nothing/will/match", vec![]),
    ];
    for (topic_name, ports) in cases {
        println!(">> [Case]: ({:?}, {:?})", topic_name, ports);
        let addrs = routes.search(&TopicName::new(topic_name.to_string()).unwrap());
        assert_eq!(addrs.len(), ports.len());
        for port in ports {
            assert_eq!(addrs.contains_key(&addr_from_port(port)), true);
        }
    }

    let unsubs = vec![
        ("xy/+", vec!["60", "61", "62"]),
    ];
    for (topic_filter, ports) in unsubs {
        for port in ports {
            routes.remove(&TopicFilter::new(topic_filter), addr_from_port(port));
        }
    }
    let cases = vec![
        ("ab", vec!["10", "20"]),
        ("ab/c", vec!["30", "31", "40"]),
        ("xy/a", vec!["70", "71"]), // [60, 61, 62] unsubscribed
        ("xy/333/t", vec!["62", "63", "70", "71"]),
        ("nothing/will/match", vec![]),
    ];
    for (topic_name, ports) in cases {
        let addrs = routes.search(&TopicName::new(topic_name.to_string()).unwrap());
        assert_eq!(addrs.len(), ports.len());
        for port in ports {
            assert_eq!(addrs.contains_key(&addr_from_port(port)), true);
        }
    }
}

#[test]
fn test_local_route_node_multiple_user() {
    /* TODO: change to LocalRoutes

    let mut routes = LocalRouteNode::new();
    let subs = vec![
    (vec![1, 2], "+", vec!["10"]),
    (vec![1], "ab", vec!["20"]),
    (vec![2], "ab/+", vec!["30", "31"]),
    (vec![2], "ab/c", vec!["40"]),
    (vec![1], "ab/d", vec!["50"]),
    (vec![2], "xy/+", vec!["60", "61", "62"]),
    (vec![1], "xy/+/t", vec!["62", "63"]),
    (vec![1], "xy/#", vec!["70", "71"]),
    (vec![1], "xy/xxxx", vec!["80"]),
];
    for &(ref user_ids, topic_filter, ref ports) in subs.iter() {
    for port in ports {
    for user_id in user_ids {
    routes.insert(*user_id, &TopicFilter::new(topic_filter),
    addr_from_port(port), QualityOfService::Level0);
}
}
}

    for user_id in vec![1, 2] {
    let addrs = routes.search(user_id, &TopicName::new("one-level-nothing".to_string()).unwrap());
    assert_eq!(addrs.len(), 1);
    assert_eq!(addrs.iter().next(), Some((&addr_from_port("10"), &QualityOfService::Level0)));
}

    assert_eq!(routes.search(2, &TopicName::new("ab".to_string()).unwrap()).len(), 1);
    let addrs = routes.search(1, &TopicName::new("ab".to_string()).unwrap());
    assert_eq!(addrs.len(), 2);
    for port in vec!["10", "20"] {
    assert_eq!(addrs.contains_key(&addr_from_port(port)), true);
}
     */
}
