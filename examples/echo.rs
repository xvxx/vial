use vial::{vial, Request, Response};

vial! {
    GET "/echo" => echo;
    POST "/echo" => post;
}

fn echo(_: Request) -> Response {
    Response::from(
        "<form method='POST'>
        <input type='text' name='echo'/>
        <input type='submit'/>
    </form>",
    )
}

fn post(req: Request) -> Response {
    Response::from(format!(
        "<h1>{}</h1>",
        req.form("echo").unwrap_or("You didn't say anything!")
    ))
}

fn main() {
    vial::run!("0.0.0.0:7667").unwrap();
}
