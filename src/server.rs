use {
    crate::{asset, Request, Response, Result, Router},
    std::{
        io::Write,
        net::{TcpListener, TcpStream, ToSocketAddrs},
        sync::{Arc, Mutex},
        time::Duration,
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
            if let Err(e) = server.handle_request(stream) {
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
    pub fn new(router: Router) -> Server {
        Server { router }
    }

    fn handle_request(&self, stream: TcpStream) -> Result<()> {
        let reader = stream.try_clone()?;

        //discard because: https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.set_read_timeout
        let _ = reader.set_read_timeout(Some(Duration::from_millis(1000)));

        let req = Request::from_reader(reader)?;
        self.write_response(stream, req)
    }

    fn write_response(&self, stream: TcpStream, req: Request) -> Result<()> {
        let panic_writer = Arc::new(Mutex::new(stream.try_clone()?));
        std::panic::set_hook(Box::new(move |info| {
            let mut res: Vec<u8> = vec![];

            Response::from(500)
                .with_body(format!("<pre>{}", info))
                .write(&mut res)
                .unwrap();

            println!("ERR 500 {}", String::from_utf8_lossy(&res));
            panic_writer.lock().unwrap().write_all(&res).unwrap();
        }));

        let method = req.method().to_string();
        let path = req.path().to_string();
        let response = self.build_response(req);

        println!("{} {} {}", method, response.code(), path);
        if response.code() == 500 {
            eprintln!("{}", response.body());
        }

        response.write(stream)
    }

    fn build_response(&self, mut req: Request) -> Response {
        if asset::exists(req.path()) {
            if let Some(req_etag) = req.header("If-None-Match") {
                if req_etag == asset::etag(req.path()).as_ref() {
                    Response::from(304)
                } else {
                    Response::from_asset(req.path())
                }
            } else {
                Response::from_asset(req.path())
            }
        } else if let Some(action) = self.router.action_for(&mut req) {
            action(req)
        } else {
            Response::from(404)
        }
    }
}
