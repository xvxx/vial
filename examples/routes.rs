use vial::prelude::*;

routes! {
    GET "/hello/:id/:name" => |req| {
        let name = req.arg("name").unwrap_or("");
        format!("Hello, {}. You're ID #{}", name, req.arg("id").unwrap_or("0"))
    };

    GET "/page/:name" => |req| {
        format!("Page: {}", req.arg("name").unwrap_or("?"))
    };

    GET "/page/:name/edit" => |req| {
        format!("Edit: {}", req.arg("name").unwrap_or("?"))
    };

    GET "/" => hello_world;
    POST "/" => redirect_to_greeting;
    GET "/:name" => hello_name;
    
    GET "/*path" => |req|
      Response::from(404).with_body(
        format!("<h1>404 Not Found: {}</h1>",
          req.arg("path").unwrap_or("")));
}

fn hello_world(_req: Request) -> &'static str {
    "<h1>Hello, world!</h1>
    <p><strong>What's your name?</strong></p>
    <form method='POST' action='/'>
        <p><input name='name' type='text'/></p>
        <p><input type='submit'/></p>
    </form>"
}

fn redirect_to_greeting(req: Request) -> Option<impl Responder> {
    let name = req.form("name")?;
    Some(Response::redirect_to(format!("/{}", name)))
}

fn hello_name(req: Request) -> String {
    format!("<h1>Why hello there, {}!</h1>", req.arg("name").unwrap())
}

fn main() {
    run!().unwrap();
}
