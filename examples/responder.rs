use {
    std::{fs, io},
    vial::prelude::*,
};

routes! {
    GET "/" => index;
    GET "/string" => from_string;
    GET "/str" => from_str;
    GET "/usize" => from_usize;
    GET "/option" => from_option;
    GET "/error" => from_error;
    GET "/error2" => from_io_error;
    GET "/panic" => from_panic;
    GET "/result" => from_result;
}

fn index(_req: Request) -> impl Responder {
    "<ul>
        <li><a href='/string'>string</a></li>
        <li><a href='/str'>str</a></li>
        <li><a href='/usize'>usize</a></li>
        <li><a href='/option'>option</a></li>
        <li><a href='/result'>result</a></li>
        <li><a href='/error'>error</a></li>
        <li><a href='/error2'>io::error</a></li>
        <li><a href='/panic'>panic</a></li>
    </ul>"
}

fn from_string(_req: Request) -> String {
    String::from("Just a <b>string</b> thing.")
}

fn from_str(_req: Request) -> &'static str {
    "Just a <i>stir</i> thur."
}

fn from_usize(_req: Request) -> usize {
    422
}

fn from_option(_req: Request) -> Option<String> {
    None
}

fn from_error(_req: Request) -> vial::Error {
    vial::Error::ConnectionClosed
}

fn from_io_error(_req: Request) -> Result<String, io::Error> {
    fs::read_to_string("made-up-file")
}

fn from_panic(_req: Request) -> impl Responder {
    panic!("ka-boom")
}

fn from_result(_req: Request) -> Result<String, vial::Error> {
    Ok("Wowsers.".to_string())
}

fn main() {
    run!().unwrap();
}
