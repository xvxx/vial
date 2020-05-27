use vial::prelude::*;

routes! {
    GET "/echo" => echo;
    POST "/echo" => post;
}

fn echo(_: Request) -> impl Responder {
    "<form method='POST'>
        <input type='text' name='echo'/>
        <input type='submit'/>
    </form>"
}

fn post(req: Request) -> impl Responder {
    format!(
        "<h1>{}</h1>",
        req.form("echo").unwrap_or("You didn't say anything!")
    )
}

fn main() {
    vial::run!().unwrap();
}
