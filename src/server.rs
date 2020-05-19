use {
    crate::{Request, Response},
    httparse,
    std::{
        io::{self, prelude::*, BufReader, Read, Write},
        net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs},
    },
    threadpool::ThreadPool,
};

const MAX_CONNECTIONS: usize = 10;
const HTTP_DATE_FMT: &str = "%a, %d %b %Y %H:%M:%S %Z";

type Result<T> = std::result::Result<T, io::Error>;

pub fn run<T: ToSocketAddrs>(addr: T, router: fn(Request) -> Response) -> Result<()> {
    let pool = ThreadPool::new(MAX_CONNECTIONS);
    let server = TcpListener::bind(&addr).expect("~ vial error: ");
    let addr = server.local_addr()?;
    println!("~ vial running at http://{}", addr);

    for stream in server.incoming() {
        let stream = stream?;
        pool.execute(move || {
            if let Err(e) = handle_request(stream, &router) {
                eprintln!("!! {}", e);
            }
        });
    }

    Ok(())
}

fn handle_request(mut stream: TcpStream, router: &fn(Request) -> Response) -> Result<()> {
    let mut reader = BufReader::new(&stream);
    let mut req = Request::new();

    loop {
        let mut headers = [httparse::EMPTY_HEADER; 100];
        let mut httpreq = httparse::Request::new(&mut headers);

        let buf = reader.fill_buf()?;
        if buf.is_empty() {
            break;
        }

        let res = match httpreq.parse(&buf) {
            Ok(res) => res,
            Err(_) => {
                write!(stream, "HTTP/1.1 400 Bad Request\r\n")?;
                return Ok(());
            }
        };

        if res.is_partial() {
            print!("{:?}", httpreq.path);
            reader = BufReader::new(&stream);
            continue;
        }

        if res.is_complete() {
            if let Some(method) = httpreq.method {
                req.method = method.to_string();
            }

            if let Some(path) = httpreq.path {
                req.path = path.to_string();
                req.parse_params();
            }

            break;
        }

        unimplemented!();
    }

    write_response(stream, req, router)?;
    Ok(())
}

fn http_current_date() -> String {
    let now = libc_strftime::epoch();
    libc_strftime::strftime_gmt(HTTP_DATE_FMT, now)
}

fn write_response(
    mut stream: TcpStream,
    req: Request,
    router: &fn(Request) -> Response,
) -> Result<()> {
    let method = req.method().to_string();
    let path = req.path().to_string();
    let res = router(req);
    let date = http_current_date();
    let content_type = "text/html; charset=utf8";
    let content_len = res.body.chars().count();
    let body = format!(
        "HTTP/1.1 200 OK\r\nServer: vial (Rust)\r\nDate: {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        date, content_type, content_len, res.body
    );
    stream.write(body.as_bytes())?;
    stream.flush()?;
    println!("{} {} {}", method, res.code, path);
    Ok(())
}
