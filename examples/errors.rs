use {std::io, vial::prelude::*};

routes! {
    GET "/" => |_|
        "<ul>
            <li><a href='/vial-error'>vial::Error</a></li>
            <li><a href='/io-error'>io::Error</a></li>
            <li><a href='/result'>result</a></li>
        </ul>";
    GET "/vial-error" => action_with_error;
    GET "/io-error" => action_with_io_error;
    GET "/result" => action_with_result;
}

fn action_with_result(_req: Request) -> Result<impl Responder, io::Error> {
    Ok("Okay!")
}

fn action_with_error(_req: Request) -> vial::Error {
    vial::Error::Other("Some vial::Error".into())
}

fn action_with_io_error(_req: Request) -> Result<String, io::Error> {
    Err(io::Error::new(io::ErrorKind::Other, "A bad io:Error!"))
}

fn main() {
    vial::run!().unwrap();
}
