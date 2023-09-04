use {
    crate::{asset, Request, Response, Result, Router},
    std::{
        io::Write,
        net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs},
        sync::{Arc, Mutex},
        thread::{spawn, JoinHandle},
    },
    threadpool::ThreadPool,
};

/// Default max number of simultaneous connections.
pub const MAX_CONNECTIONS: usize = 10;

/// Starts a new Vial server. Should always be invoked via the
/// [`vial::run!()`](macro.run.html) macro, since there is some setup
/// that needs to happen.
#[doc(hidden)]
pub fn run<T: ToSocketAddrs>(addr: T, router: Router, banner: Option<&str>) -> Result<()> {
    let mut addr = addr.to_socket_addrs()?;
    let addr = addr.next().unwrap();
    let mut server = Server::new(router, addr, MAX_CONNECTIONS, banner);
    server.run();
    Ok(())
}

/// Starts a new Vial server accepting only one request. Should always
/// be invoked via the [`vial::run_once!()`](macro.run_once.html) macro,
/// since there is some setup that needs to happen.
#[doc(hidden)]
pub fn run_once<T: ToSocketAddrs>(addr: T, router: Router) -> Result<(String, JoinHandle<()>)> {
    let mut addr = addr.to_socket_addrs()?;
    let addr = addr.next().unwrap();
    let mut server = Server::new(router, addr, 1, None);
    let addr = format!("http://{}/", server.addr().to_string());
    let thread = spawn(move || server.run_once());
    Ok((addr, thread))
}

/// A Vial server instance. Has its own threadpool to handle incoming connections.
pub struct Server<'s> {
    router: Arc<Router>,
    listener: TcpListener,
    pool: ThreadPool,
    banner: Option<&'s str>,
}

impl<'s> Server<'s> {
    /// Creates a new Vial server instance. This immediately binds the given address, but does not
    /// start accepting incoming traffic until `run` is called.
    ///
    /// This will panic upon being unable to bind the given IP and port combination.
    pub fn new(
        router: Router,
        addr: SocketAddr,
        num_threads: usize,
        banner: Option<&'s str>,
    ) -> Server {
        let listener = TcpListener::bind(&addr)
            .expect(&format!("unable to bind to addr: {}", addr.to_string()));
        Server {
            router: Arc::new(router),
            listener,
            pool: ThreadPool::new(num_threads),
            banner,
        }
    }

    /// Returns the local address the server is bound to.
    pub fn addr(&self) -> SocketAddr {
        // SAFETY: Safe, since the listener is already bound at this point
        self.listener.local_addr().unwrap()
    }

    /// Prints the banner info to stdout.
    fn print_banner(&self) {
        #[cfg(feature = "state")]
        eprintln!("! vial feature `state` is now built-in. You can safely remove it.");

        if let Some(banner) = self.banner {
            if !banner.is_empty() {
                println!(
                    "{}",
                    banner.replace("{}", &format!("http://{}", self.addr()))
                );
            }
        } else {
            println!("~ vial running at http://{}", self.addr());
        }
    }

    /// Runs this server instance indefinitely.
    pub fn run(&mut self) {
        self.print_banner();

        for stream in self.listener.incoming() {
            let router = self.router.clone();
            match stream {
                Ok(stream) => {
                    self.pool.execute(move || {
                        if let Err(e) = Self::handle_request(stream, router) {
                            eprintln!("!! {}", e);
                        }
                    });
                }
                Err(_) => eprintln!("!! connection failed"),
            };
        }
    }

    /// Runs the server instance for one incoming request. Useful for mock testing.
    pub fn run_once(&mut self) {
        self.print_banner();

        match self.listener.accept() {
            Ok((stream, _)) => {
                if let Err(e) = Self::handle_request(stream, self.router.clone()) {
                    eprintln!("!! {}", e);
                }
            }

            Err(_) => eprintln!("!! connection failed"),
        };
    }

    fn handle_request(stream: TcpStream, router: Arc<Router>) -> Result<()> {
        let reader = stream.try_clone()?;
        let req = Request::from_reader(reader)?;
        Self::write_response(stream, req, router)
    }

    fn write_response(stream: TcpStream, req: Request, router: Arc<Router>) -> Result<()> {
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
        let response = Self::build_response(req, router);

        println!("{} {} {}", method, response.code(), path);
        if response.code() == 500 {
            eprintln!("{}", response.body());
        }

        response.write(stream)
    }

    fn build_response(mut req: Request, router: Arc<Router>) -> Response {
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
        } else if let Some(action) = router.action_for(&mut req) {
            action(req)
        } else {
            Response::from(404)
        }
    }
}
