use {
    crate::{asset, Request, Response, Result},
    httparse,
    std::{
        io::{self, prelude::*, BufReader, Read, Write},
        net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs},
        sync::{Arc, Mutex},
    },
    threadpool::ThreadPool,
};

const MAX_CONNECTIONS: usize = 10;
const HTTP_DATE_FMT: &str = "%a, %d %b %Y %H:%M:%S %Z";

type Router = Arc<Mutex<crate::Router>>;

pub fn run<T: ToSocketAddrs>(addr: T, router: Router) -> Result<()> {
    let pool = ThreadPool::new(MAX_CONNECTIONS);
    let server = TcpListener::bind(&addr)?;
    let addr = server.local_addr()?;
    println!("~ vial running at http://{}", addr);

    for stream in server.incoming() {
        let router = router.clone();
        let stream = stream?;
        pool.execute(move || {
            if let Err(e) = handle_request(stream, &router) {
                eprintln!("!! {}", e);
            }
        });
    }

    Ok(())
}

fn handle_request(mut stream: TcpStream, router: &Router) -> Result<()> {
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

    write_response(stream, req, router)?;
    Ok(())
}

fn http_current_date() -> String {
    let now = libc_strftime::epoch();
    libc_strftime::strftime_gmt(HTTP_DATE_FMT, now)
}

fn write_response(mut stream: TcpStream, req: Request, router: &Router) -> Result<()> {
    let method = req.method().to_string();
    let path = req.path().to_string();
    let router = router.lock().unwrap();

    let res = if asset::exists(req.path()) {
        if let Some(req_etag) = req.header("If-None-Match") {
            if req_etag == asset::hash(req.path()) {
                Response::from(304)
            } else {
                Response::from_file(req.path())
            }
        } else {
            Response::from_file(req.path())
        }
    } else if let Some(action) = router.action_for(&req) {
        action(req)
    } else {
        Response::from(404).with_body("404 Not Found")
    };

    let mut header = format!(
        "HTTP/1.1 {} OK\r\nServer: vial (Rust)\r\nDate: {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n",
        res.code, http_current_date(), res.content_type, res.len(),
    );

    header.push_str(
        &res.headers
            .iter()
            .map(|(key, val)| format!("{}: {}", key, val))
            .collect::<Vec<_>>()
            .join("\r\n"),
    );

    if !header.ends_with("\r\n") {
        header.push_str("\r\n");
    }
    header.push_str("\r\n");

    stream.write(header.as_bytes())?;
    if res.buf.is_empty() {
        stream.write(res.body.as_bytes())?;
    } else {
        stream.write(&res.buf)?;
    }
    stream.flush()?;
    println!("{} {} {}", method, res.code, path);
    Ok(())
}
