use {
    crate::{Responder, Response},
    horrorshow::{FnRenderer, TemplateBuffer},
    std::fmt,
};

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
        ::horrorshow::html! { $($t)* }
    };
}

#[macro_export]
macro_rules! owned_html {
    ($($t:tt)*) => {
        ::horrorshow::owned_html! { $($t)* }
    };
}

#[macro_export]
macro_rules! box_html {
    ($($t:tt)*) => {
        ::horrorshow::box_html! { $($t)* }
    };
}
