use route_match::route;

#[test]
fn test_route_by_method() {
    fn route(method: &str, path: &str) -> u8 {
        route! {
            match (&method, &path) {
                GET /foo => 1,
                POST /foo => 2,
                OPTIONS /foo => 3,
                _ => 4,
            }
        }
    }

    assert_eq!(route("GET", "/foo"), 1);
    assert_eq!(route("GET", "foo"), 1);
    assert_eq!(route("POST", "/foo"), 2);
    assert_eq!(route("OPTIONS", "/foo"), 3);
    assert_eq!(route("CONNECT", "/foo"), 4);
    assert_eq!(route("GET", "foo/bar"), 4);
}

#[test]
fn test_route_by_path() {
    fn route(method: &str, path: &str) -> u8 {
        route! {
            match (&method, &path) {
                GET /foo/bar/baz => 5,
                GET /foo => 1,
                GET /bar => 2,
                GET /baz => 3,
                _ => 4,
            }
        }
    }

    assert_eq!(route("GET", "/foo"), 1);
    assert_eq!(route("GET", "/bar"), 2);
    assert_eq!(route("GET", "/baz"), 3);
    assert_eq!(route("GET", "/foo/bar"), 4);
    assert_eq!(route("GET", "/foo/bar/baz"), 5);
    assert_eq!(route("POST", "/foo"), 4);
}

#[test]
fn test_path_args() {
    fn route(method: &str, path: &str) -> String {
        route! {
            match (&method, &path) {
                GET /foo/bar => "static".to_string(),
                GET /foo/:arg1/bar/:arg2 => format!("{}:{}", arg1, arg2),
                GET /foo/:arg => arg.to_string(),
                _ => "none".to_string(),
            }
        }
    }

    assert_eq!(&route("GET", "/foo/bar"), "static");
    assert_eq!(&route("GET", "/foo/baz"), "baz");
    assert_eq!(&route("GET", "/foo/foo/bar/bar"), "foo:bar");
    assert_eq!(&route("GET", "/baz"), "none");
}

#[test]
fn test_wildcard() {
    fn route(method: &str, path: &str) -> String {
        route! {
            match (&method, &path) {
                GET /foo/.. => "static".to_string(),
                GET /bar/..:rest => rest.to_string(),
                _ => "none".to_string(),
            }
        }
    }

    assert_eq!(&route("GET", "/foo/bar/baz"), "static");
    assert_eq!(&route("GET", "/bar/baz"), "baz");
    assert_eq!(&route("GET", "/bar/foo/bar/baz"), "foo/bar/baz");
    assert_eq!(&route("GET", "/baz"), "none");
}

#[test]
fn test_any_method() {
    fn route(method: &str, path: &str) -> String {
        route! {
            match (&method, &path) {
                GET /foo => "GET".to_string(),
                _ /foo => "any_method".to_string(),
                _ => "none".to_string(),
            }
        }
    }

    assert_eq!(&route("GET", "/foo"), "GET");
    assert_eq!(&route("POS", "/foo"), "any_method");
    assert_eq!(&route("POST", "/bar/baz"), "none");
    assert_eq!(&route("GET", "/baz"), "none");
}
