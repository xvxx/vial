use vial::prelude::*;

routes! {
    POST "/json" => post;
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
