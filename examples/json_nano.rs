use nanoserde::{DeJson, SerJson};
use vial::prelude::*;

routes! {
    GET "/" => form;
    POST "/json" => post;
}

fn form(_: Request) -> impl Responder {
    r#"Do this: <pre>
curl -d '{"message": "Hello, Vial!"}' -H "Content-Type: application/json" -X POST http://0.0.0.0:7667/json
"#
}

fn post(req: Request) -> impl Responder {
    #[derive(SerJson, DeJson)]
    struct Message {
        message: String,
    }

    match req.json::<Message>().ok().map(|m| m.message) {
        Some(message) => Response::from(200).with_json(Message {
            message: format!("Echo: {}", message),
        }),
        None => Response::from(400).with_body("json request parse error"),
    }
}

fn main() {
    vial::run!().unwrap();
}
