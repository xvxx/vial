use vial::prelude::*;

routes! {
    GET "/" => |_| r#"
        <ul>
            <li><a href='/boom'>panic!("boom!")</a></li>
            <li>
                <a href='javascript:window.location = "/custom?msg=" + prompt("Please enter your panic! message");'>
                    panic!(your message here)
                </a>
            </li>
        </ul>
        "#;
    GET "/boom" => boom;
    GET "/custom" => custom;
}

fn boom(_req: Request) -> impl Responder {
    panic!("boom!")
}

fn custom(req: Request) -> impl Responder {
    panic!("{}", req.query("msg").unwrap_or("custom"))
}

fn main() {
    vial::run!().unwrap();
}
