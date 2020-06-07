use {
    crate::{asset, Request, Response, Result, Router},
    std::{
        net::{TcpListener, TcpStream, ToSocketAddrs},
        sync::Arc,
    },
    threadpool::ThreadPool,
};

const MAX_CONNECTIONS: usize = 10;

/// Starts a new Vial server. Should always be invoked via the
/// [`vial::run!()`](macro.run.html) macro, since there is some setup
/// that needs to happen.
#[doc(hidden)]
pub fn run<T: ToSocketAddrs>(addr: T, router: Router, banner: Option<&str>) -> Result<()> {
    let pool = ThreadPool::new(MAX_CONNECTIONS);
    let listener = TcpListener::bind(&addr)?;
    let addr = listener.local_addr()?;
    let server = Arc::new(Server::new(router));
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
        let req = Request::from_reader(reader)?;
        self.write_response(stream, req)
    }

    fn write_response(&self, stream: TcpStream, req: Request) -> Result<()> {
        let method = req.method().to_string();
        let path = req.path().to_string();
        let response = self.build_response(req);

        println!("{} {} {}", method, response.code(), path);
        if response.code() == 500 {
            eprintln!("{}", response.body);
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
