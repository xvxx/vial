use {
    minreq::{Method, Request},
    std::{
        net::{Ipv4Addr, SocketAddrV4},
        thread,
    },
    vial::{Response, Router, Server},
};

fn make_request(router: Router, method: Method, url: &str) -> minreq::Response {
    let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0);
    let mut server = Server::new(router, addr.into(), 1, None);
    let request_addr = format!("http://{}{}", server.addr(), url);
    let request = Request::new(method, request_addr);
    thread::spawn(move || {
        server.run_once();
    });
    let resp = request.send().unwrap();

    resp
}

fn string_body(_: vial::Request) -> Response {
    "Hello, World!".into()
}

#[test]
fn mock_get() {
    let mut router = Router::new();
    router.insert("GET", "/get", string_body);
    let resp = make_request(router, Method::Get, "/get");
    assert_eq!(resp.status_code, 200);
    assert_eq!(resp.as_str().unwrap(), "Hello, World!")
}

#[test]
fn mock_post() {
    let mut router = Router::new();
    router.insert("POST", "/post", string_body);
    let resp = make_request(router, Method::Post, "/post");
    assert_eq!(resp.status_code, 200);
    assert_eq!(resp.as_str().unwrap(), "Hello, World!")
}
