use {
    crate::{Responder, Response},
    horrorshow::{FnRenderer, TemplateBuffer},
    std::fmt,
};

pub use horrorshow as horrowshow_crate;

impl<F> Responder for FnRenderer<F>
where
    FnRenderer<F>: fmt::Display,
    F: FnOnce(&mut TemplateBuffer<'_>),
{
    fn to_response(self) -> Response {
        Response {
            body: self.to_string(),
            ..Response::default()
        }
    }
}

#[macro_export]
macro_rules! html {
    ($($t:tt)*) => {
        ::vial::features::horrorshow::horrowshow_crate::html! { $($t)* }
    };
}

#[macro_export]
macro_rules! owned_html {
    ($($t:tt)*) => {
        ::vial::features::horrorshow::horrowshow_crate::owned_html! { $($t)* }
    };
}

#[macro_export]
macro_rules! box_html {
    ($($t:tt)*) => {
        ::vial::features::horrorshow::horrowshow_crate::box_html! { $($t)* }
    };
}
