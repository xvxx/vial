use std::fmt;
use vial::prelude::*;

routes! {
    GET "/" => |_| "<a href='/profile'>Profile</a>";
    GET "/login" => |_| "
    <form action='POST'>
        <p><input type='text' name='login' placeholder='Login'/></p>
        <p><input type='password' name='password' placeholder='Password'/></p>
        <p><input type='submit'/></p>
    </form>
    ";
    POST "/login" => login;
    #[filter(login_required)]
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

fn login_required(req: &mut Request) -> Option<Response> {
    if req.cache(current_user).as_ref().is_some() {
        None
    } else {
        Some(Response::redirect_to("/login"))
    }
}

fn current_user(req: &Request) -> Option<User> {
    let header = req.query("username")?;
    Some(User::new(header))
}

fn profile(req: Request) -> Option<impl Responder> {
    let user = req.cache(current_user).as_ref()?;
    Some(format!("<b>{}</b>", user))
}

fn login(_req: Request) -> impl Responder {
    unimplemented!()
}

fn main() {
    run!().unwrap()
}
