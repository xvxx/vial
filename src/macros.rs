#[macro_export]
macro_rules! run {
    () => {
        vial::run("0.0.0.0:7667", vec![vial_recognize]);
    };
    ($addr:expr) => {
        vial::run($addr, vec![vial_recognize]);
    };
    ($addr:expr, $($module:ident),+) => {
        vial::run($addr, vec![$($module::vial_recognize),+]);
    };

}

#[macro_export]
macro_rules! vial {
    ( $($method:ident $path:expr => $body:expr;)* ) => {
        fn vial_check_method() {
            #![allow(non_snake_case)]
            fn GET() {}
            fn POST() {}
            fn PUT() {}
            fn DELETE() {}
            fn UPDATE() {}
            fn PATCH() {}
            $($method();)*
        }


        pub(crate) fn vial_recognize(
            req: &::vial::Request
        ) -> Option<fn(::vial::Request) -> ::vial::Response> {
            match (req.method(), req.path()) {
                $( (stringify!($method), $path) => Some($body), )*
                _ => None,
            }
        }
    };
}

macro_rules! error {
    ($msg:expr) => {
        std::io::Error::new(std::io::ErrorKind::Other, $msg)
    };
    ($fmt:expr, $($args:expr),*) => {
        error!(format!($fmt, $($args),*))
    };
}
