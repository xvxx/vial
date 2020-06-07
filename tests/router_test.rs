use vial::{Request, Response, Router};

fn show(r: Request) -> Response {
    format!("Show: {}", r.arg("page").unwrap_or("?")).into()
}
fn show_raw(r: Request) -> Response {
    format!("Raw: {}", r.arg("page").unwrap_or("?")).into()
}
fn show_parts(r: Request) -> Response {
    format!("Parts: {}", r.arg("parts").unwrap_or("?")).into()
}
fn show_mix(r: Request) -> Response {
    format!(
        "Mix: {} {}",
        r.arg("of").unwrap_or("?"),
        r.arg("things").unwrap_or("?")
    )
    .into()
}
fn about(_: Request) -> Response {
    "About".into()
}
fn info(_: Request) -> Response {
    "Info".into()
}

#[test]
fn routing() {
    let mut router = Router::new();
    router.insert("GET", "/about", about);
    router.insert("GET", "/:page", show);
    router.insert("GET", "/info", info);
    router.insert("GET", "/:page.md", show_raw);
    router.insert("GET", "/mix/:of/*things", show_mix);
    router.insert("GET", "/*parts", show_parts);

    let mut req = Request::from_path("/");
    assert_eq!(router.action_for(&mut req), None);

    let mut req = Request::from_path("/cats");
    assert_eq!(
        router.action_for(&mut req).unwrap()(req).to_string(),
        "Show: cats".to_string()
    );

    let mut req = Request::from_path("/dogs");
    assert_eq!(
        router.action_for(&mut req).unwrap()(req).to_string(),
        "Show: dogs".to_string()
    );

    let mut req = Request::from_path("/rabbits?haxcode=1");
    assert_eq!(
        router.action_for(&mut req).unwrap()(req).to_string(),
        "Show: rabbits".to_string()
    );

    let mut req = Request::from_path("/lemurs/?other-haxcode=1&bobby=brown");
    assert_eq!(
        router.action_for(&mut req).unwrap()(req).to_string(),
        "Show: lemurs".to_string()
    );

    let mut req = Request::from_path("/about");
    assert_eq!(
        router.action_for(&mut req).unwrap()(req).to_string(),
        "About".to_string()
    );

    let mut req = Request::from_path("/info");
    assert_eq!(
        router.action_for(&mut req).unwrap()(req).to_string(),
        "Show: info".to_string()
    );

    let mut req = Request::from_path("/cats.md");
    assert_eq!(
        router.action_for(&mut req).unwrap()(req).to_string(),
        "Raw: cats".to_string()
    );

    let mut req = Request::from_path("/cats and dogs.md");
    assert_eq!(
        router.action_for(&mut req).unwrap()(req).to_string(),
        "Raw: cats and dogs".to_string()
    );

    let mut req = Request::from_path("/slashes/dont/match");
    assert_eq!(
        router.action_for(&mut req).unwrap()(req).to_string(),
        "Parts: slashes/dont/match".to_string()
    );

    let mut req = Request::from_path("/mix/o/magic/i/see");
    assert_eq!(
        router.action_for(&mut req).unwrap()(req).to_string(),
        "Mix: o magic/i/see".to_string()
    );

    let mut req = Request::from_path("/mix/of/Cargo.toml");
    assert_eq!(
        router.action_for(&mut req).unwrap()(req).to_string(),
        "Mix: of Cargo.toml".to_string()
    );
}
