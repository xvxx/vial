use hatter::prelude::*;
use vial::prelude::*;

fn render(src: &str) -> hatter::Result<String> {
    let mut env = Env::new();
    env.render(src)
}

routes! {
    GET "/" => |_| render(r#"
<p> You're looking for this:
    <a href='/echo'> echo
"#);
    GET "/echo" => echo;
    POST "/echo" => post;
}

fn echo(_: Request) -> impl Responder {
    render(
        r#"
<form POST="">
    <p> Type something:
        <input:text@echo/>
        <input:submit/>
"#,
    )
}

fn post(req: Request) -> impl Responder {
    let mut env = Env::new();
    env.set(
        "echo",
        req.form("echo").unwrap_or("You didn't say anyting!"),
    );
    env.render("<h1> echo")
}

fn main() {
    vial::run!("0.0.0.0:7667").unwrap();
}
