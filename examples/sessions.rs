use vial::prelude::*;

routes! {
    GET "/" => show;
    GET "/clear" => clear;
    GET "/set" => set;
}

fn show(req: Request) -> impl Responder {
    let count: usize = req.session("count").unwrap_or("0").parse().unwrap();
    let new_count = count + 1;
    Response::from_session("count", &new_count.to_string()).with_body(format!(
        r#"
<h1> Count: {} </h1>
<p><a href="/clear">Clear Count</a></p>
<form action="/set" method="GET">
    <input type="text" name="count" />
    <input type="submit" value="Set Count" />
</form>
    "#,
        new_count
    ))
}

fn clear(_req: Request) -> impl Responder {
    Response::redirect_to("/").without_session("count")
}

fn set(req: Request) -> impl Responder {
    let mut res = Response::redirect_to("/");
    if let Some(val) = req.query("count").map(|c| c.parse::<usize>().unwrap_or(0)) {
        let val = val.to_string();
        res.set_session("count", &val);
    }
    res
}

fn main() {
    run!().unwrap();
}
