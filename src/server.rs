use std::time::Duration;

use crate::Compression;
use {
    crate::{asset, Request, Response, Result, Router},
    std::{
        io::Write,
        net::{TcpListener, TcpStream, ToSocketAddrs},
        sync::{Arc, Mutex},
    }
};

/// Starts a new Vial server. Should always be invoked via the
/// [`vial::run!()`](macro.run.html) macro, since there is some setup
/// that needs to happen.
#[doc(hidden)]
pub fn run<T: ToSocketAddrs>(addr: T, router: Router, banner: Option<&str>) -> Result<()> {
    // If a range is supplied, the lower bound will be the core pool size while the upper bound will be a maximum pool size the pool is allowed to burst up to when the core threads are busy.
    let pool = threadfin::builder().size(1..=128).build();
    let listener = TcpListener::bind(&addr)?;
    let addr = listener.local_addr()?;
    let server = Arc::new(Server::new(router));
    #[cfg(feature = "state")]
    eprintln!("! vial feature `state` is now built-in. You can safely remove it.");

    if let Some(banner) = banner {
        if !banner.is_empty() {
            println!("{}", banner.replace("{}", &format!("http://{}", addr)));
        }
    } else {
        println!("~ vial running at http://{}", addr);
    }

    for stream in listener.incoming() {
        let server = server.clone();
        let stream = stream?;
        pool.execute(move || {
            if let Err(e) = server.handle_request(&stream) {
                eprintln!("!! {}", e);
            }
        });
    }

    Ok(())
}

struct Server {
    router: Router,
}

impl Server {
    pub fn new(router: Router) -> Self {
        Self { router }
    }

    fn handle_request(&self, stream: &TcpStream) -> Result<()> {
        let stream = stream.try_clone()?;
        // discard because: https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.set_read_timeout
        // "An Err is returned if the zero Duration is passed to this method." thus no need to check result
        drop(stream.set_read_timeout(Some(Duration::from_millis(1000))));
        let mut req = Request::from_stream(&stream)?;
        req.set_remote_addr(stream.peer_addr()?.to_string());
        self.write_response(stream, req)
    }

    fn write_response(&self, stream: TcpStream, req: Request) -> Result<()> {
        let compression: Option<Compression> = None;
        #[cfg(feature = "compression")]
        let compression = req.compression();
        let panic_writer = Arc::new(Mutex::new(stream.try_clone()?));
        std::panic::set_hook(Box::new(move |info| {
            let mut response: Vec<u8> = vec![];
            let write = Response::from(500)
                .with_body(format!("<pre>{}", info))
                .write(&mut response, &compression);
            if write.is_ok() {
                println!("ERR 500 {}", String::from_utf8_lossy(&response));
                if let Ok(mut panic_writer) = panic_writer.lock() {
                    if panic_writer.write_all(&response).is_ok() {
                    } else {
                        println!("Error writing 500 server error to TCP connection");
                    }
                } else {
                    println!("Error locking panic writer");
                }
            }
        }));

        let method = req.method().to_string();
        let path = req.path().to_string();
        let (response, encoding) = self.build_response(req);

        println!("{} {} {}", method, response.code(), path);
        if response.code() == 500 {
            eprintln!("{}", response.body());
        }
        response.write(stream, &encoding)
    }

    fn build_response(&self, mut req: Request) -> (Response, Option<Compression>) {
        let encoding: Option<Compression> = None;
        #[cfg(feature = "compression")]
        let encoding = req.compression();
        //Should this really check for a file on every request? Maybe only if the router doesn't have an action..?
        if asset::exists(req.path()) {
            req.header("If-None-Match").map_or_else(
                || (Response::from_asset(req.path()), encoding.clone()),
                |req_etag| {
                    if req_etag == asset::etag(req.path()).as_ref() {
                        (Response::from(304), encoding.clone())
                    } else {
                        (Response::from_asset(req.path()), encoding.clone())
                    }
                },
            )
        } else if let Some(action) = self.router.action_for(&mut req) {
            let gzip = encoding;
            (action(req), gzip)
        } else {
            (Response::from(404), encoding)
        }
    }
}
