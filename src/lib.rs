#![allow(unused)]

mod method;
mod request;
mod response;
mod server;
mod util;

pub use {request::Request, response::Response, server::run};

#[macro_export]
macro_rules! run {
    ($addr:expr, $module:ident) => {
        vial::run($addr, $module::vial_router);
    };
    ($addr:expr) => {
        vial::run($addr, vial_router);
    };
}

#[macro_export]
macro_rules! vial {
    ( $($method:ident $path:expr => $body:expr;)* ) => {
        fn vial_check_method() {
            #[allow(non_snake_case)]
            fn GET() {}
            #[allow(non_snake_case)]
            fn POST() {}
            $($method();)*
        }


        pub fn vial_router(req: ::vial::Request) -> ::vial::Response {
            match (req.method(), req.path()) {
                $( (stringify!($method), $path) => $body(req), )*
                _ => ::vial::Response::from("404 Not Found"),
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
