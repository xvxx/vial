use std::fmt;
use vial::prelude::*;

routes! {
    GET "/" => |_| "<a href='/profile'>Profile</a>";
    GET "/profile" => profile;
}

#[derive(Debug)]
struct User {
    name: String,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl User {
    fn new<S: AsRef<str>>(name: S) -> User {
        User {
            name: name.as_ref().to_string(),
        }
    }
}

fn current_user(req: &Request) -> Option<User> {
    let header = req.header("X-User")?;
    Some(User::new(header))
}

fn profile(req: Request) -> Option<impl Responder> {
    let user = req.cache(current_user).as_ref()?;
    Some(format!("<b>{}</b>", user))
}

fn main() {
    run!().unwrap()
}
