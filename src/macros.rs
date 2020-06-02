#[macro_export]
macro_rules! run {
    () => {
        vial::run!("0.0.0.0:7667")
    };
    ($addr:expr) => {{
        vial::run!($addr, self)
    }};
    ($($module:ident),+) => {{
        vial::run!("0.0.0.0:7667", $($module),+)
    }};
    ($addr:expr, $($module:ident),+) => {{
        vial::setup!();
        let mut router = ::vial::Router::new();
        $($module::vial_add_to_router(&mut router);)+
        vial::run($addr, router)
    }};
}

#[cfg(feature = "stateful")]
#[macro_export]
macro_rules! use_state {
    ($state:expr) => {
        vial::storage::init();
        vial::storage::set($state);
    };
}

#[macro_export]
macro_rules! setup {
    () => {
        #[cfg(bundle_assets)]
        #[macro_export]
        macro_rules! vial_bundled_assets {
            () => { include!(concat!(env!("OUT_DIR"), "/bundle.rs")) };
        }
        #[cfg(bundle_assets)]
        vial::include_assets!();
        vial::asset_dir!(@option option_env!("ASSET_DIR"));
    };
}

#[macro_export]
macro_rules! include_assets {
    () => {
        unsafe {
            vial::BUNDLED_ASSETS = Some(vial_bundled_assets!());
        }
    };
}

#[macro_export]
macro_rules! asset_dir {
    (@option $opt:expr) => {
        if let Some(dir) = $opt {
            ::vial::asset_dir!(dir);
        }
    };
    ($dir:expr) => {
        unsafe {
            ::vial::ASSET_DIR = Some($dir);
        }
    };
}

#[macro_export]
macro_rules! bundle_assets {
    ($dir:expr) => {
        ::vial::bundle_assets($dir)
    };
}

#[macro_export]
macro_rules! routes {
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

        pub fn vial_add_to_router(router: &mut ::vial::Router) {
            $( router.insert(::vial::Method::$method, $path, |req| {
                use ::vial::Responder;
                #[cfg(not(feature = "stateful"))]
                let b: fn(::vial::Request) -> _ = $body;
                #[cfg(feature = "stateful")]
                let b: fn(::vial::State<_>) -> _ = $body;
                b(req.into()).to_response()
            }); )*
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
