#![allow(unused)]

mod method;
mod request;
mod response;
mod server;

pub use {request::Request, response::Response, server::run};

#[macro_export]
macro_rules! vial {
    ( $($method:ident $path:expr => $body:expr;)* ) => {
        fn check() {
            #[allow(non_snake_case)]
            fn GET() {}
            #[allow(non_snake_case)]
            fn POST() {}
            $($method();)*
        }


        fn route(req: Request) -> Response {
            match (req.method(), req.path()) {
                $( (stringify!($method), $path) => $body(req), )*
                _ => Response::from("404 Not Found"),
            }
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
