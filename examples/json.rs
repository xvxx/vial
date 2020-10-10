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
    match req
        .json::<serde_json::Value>()
        .ok()
        .as_ref()
        .and_then(|val| val.as_object())
        .and_then(|obj| obj.get("message"))
        .and_then(|val| val.as_str())
        .map(|message| message.to_string())
    {
        Some(message) => Response::from(200).with_json(serde_json::json!({
            "message": format!("Echo: {}", message)
        })),
        None => Response::from(400).with_body("json request parse error"),
    }
}

fn main() {
    vial::run!().unwrap();
}
