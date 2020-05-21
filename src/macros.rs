#[macro_export]
macro_rules! run {
    () => {
        vial::run!("0.0.0.0:7667")
    };
    ($addr:expr) => {
        vial::run($addr, ::std::sync::Arc::new(::std::sync::Mutex::new(vec![vial_router()])))
    };
    ($addr:expr, $($module:ident),+) => {
        vial::run($addr, ::std::sync::Arc::new(::std::sync::Mutex::new(vec![$($module::vial_router()),+])))
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

        pub(crate) fn vial_router() -> ::vial::Router {
            let mut router = ::vial::Router::new();
            $( router.insert(::vial::Method::$method, $path, $body); )*
            router
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
