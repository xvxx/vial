use {
    crate::{Request, Response, Result},
    httparse,
    std::{
        io::{self, prelude::*, BufReader, Read, Write},
        net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs},
    },
    threadpool::ThreadPool,
};

const MAX_CONNECTIONS: usize = 10;
const HTTP_DATE_FMT: &str = "%a, %d %b %Y %H:%M:%S %Z";

type Routers = Vec<fn(&Request) -> Option<fn(Request) -> Response>>;

pub fn run<T: ToSocketAddrs>(addr: T, routers: Routers) -> Result<()> {
    let pool = ThreadPool::new(MAX_CONNECTIONS);
    let server = TcpListener::bind(&addr).expect("~ vial error: ");
    let addr = server.local_addr()?;
    println!("~ vial running at http://{}", addr);

    for stream in server.incoming() {
        let stream = stream?;
        let routers = routers.clone();
        pool.execute(move || {
            if let Err(e) = handle_request(stream, &routers) {
                eprintln!("!! {}", e);
            }
        });
    }

    Ok(())
}

fn handle_request(mut stream: TcpStream, routers: &Routers) -> Result<()> {
    let mut buffer = vec![];
    let mut read_buf = [0u8; 512];

    let req = loop {
        let n = stream.read(&mut read_buf)?;
        if n == 0 {
            return Err(error!("Empty response"));
        }
        buffer.extend_from_slice(&read_buf[..n]);
        if let Some(req) = Request::from_raw_http_request(&mut buffer)? {
            break req;
        }
    };

    write_response(stream, req, routers)?;
    Ok(())
}

fn http_current_date() -> String {
    let now = libc_strftime::epoch();
    libc_strftime::strftime_gmt(HTTP_DATE_FMT, now)
}

fn write_response(mut stream: TcpStream, req: Request, routers: &Routers) -> Result<()> {
    let method = req.method().to_string();
    let path = req.path().to_string();
    let res = if let Some(router) = routers.iter().find_map(|r| r(&req)) {
        router(req)
    } else {
        Response::from(404).with_body("404 Not Found")
    };

    let date = http_current_date();
    let content_type = "text/html; charset=utf8";
    let content_len = res.body.len();
    let body = format!(
        "HTTP/1.1 {} OK\r\nServer: vial (Rust)\r\nDate: {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        res.code, date, content_type, content_len, res.body
    );
    stream.write(body.as_bytes())?;
    stream.flush()?;
    println!("{} {} {}", method, res.code, path);
    Ok(())
}
