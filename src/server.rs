use std::time::Duration;

use crate::Compression;
use {
    crate::{asset, Request, Response, Result, Router},
    std::{
        io::Write,
        net::{TcpListener, TcpStream, ToSocketAddrs},
        sync::{Arc, Mutex},
    },
    threadpool::ThreadPool,
};

const INITIAL_THREADS: usize = 10;
const MAXIMUM_THREADS: usize = 400;

/// Starts a new Vial server. Should always be invoked via the
/// [`vial::run!()`](macro.run.html) macro, since there is some setup
/// that needs to happen.
#[doc(hidden)]
pub fn run<T: ToSocketAddrs>(addr: T, router: Router, banner: Option<&str>) -> Result<()> {
    let mut pool = ThreadPool::new(INITIAL_THREADS);
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
        // if all threads are active, extend by two
        if pool.active_count() > pool.max_count() - 1 && pool.max_count() < MAXIMUM_THREADS {
            pool.set_num_threads(pool.max_count() + 2);
        }
        // tldr: if no active threads and the threadpool has
        // been expanded, halve the total number of threads.
        if pool.active_count() == 0 && pool.max_count() > INITIAL_THREADS * 2 {
            pool.set_num_threads(pool.max_count() / 2);
        }

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
        let _compression: Option<Compression> = None;
        #[cfg(feature = "compression")]
        let _compression = req.compression();
        let panic_writer = Arc::new(Mutex::new(stream.try_clone()?));
        std::panic::set_hook(Box::new(move |info| {
            let mut response: Vec<u8> = vec![];
            Response::from(500)
                .with_body(format!("<pre>{}", info))
                .write(&mut response, &_compression)
                .unwrap();

            println!("ERR 500 {}", String::from_utf8_lossy(&response));
            panic_writer.lock().unwrap().write_all(&response).unwrap();
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
        let _encoding: Option<Compression> = None;
        #[cfg(feature = "compression")]
        let _encoding = req.compression();
        //Should this really check for a file on every request? Maybe only if the router doesn't have an action..?
        if asset::exists(req.path()) {
            if let Some(req_etag) = req.header("If-None-Match") {
                if req_etag == asset::etag(req.path()).as_ref() {
                    (Response::from(304), _encoding)
                } else {
                    (Response::from_asset(req.path()), _encoding)
                }
            } else {
                (Response::from_asset(req.path()), _encoding)
            }
        } else if let Some(action) = self.router.action_for(&mut req) {
            let gzip = _encoding;
            (action(req), gzip)
        } else {
            (Response::from(404), _encoding)
        }
    }
}
