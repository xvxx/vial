mod wiki {
    vial::routes! {
        GET "/wiki" => |_| "This is the wiki.";
    }
}

vial::routes! {
    GET "/" => |_| "Index page.";
}

mod mods;
use mods::blog;

fn main() {
    vial::run!(self, blog, wiki).unwrap();
}
