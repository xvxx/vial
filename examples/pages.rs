use vial::prelude::*;

routes! {
    GET "/" => list;
}

#[derive(Debug, Clone)]
pub struct Page {
    name: String,
}

impl Page {
    fn new(name: &str) -> Page {
        Page {
            name: name.to_string(),
        }
    }
}

mod db {
    use super::*;

    pub fn lookup(_query: &str) -> Vec<Page> {
        vec![Page::new("Index"), Page::new("Help"), Page::new("About")]
    }
}

struct PageNames(Vec<String>);

fn all_pages(_: &Request) -> Vec<Page> {
    db::lookup("select * from pages")
}

fn page_names(req: &Request) -> PageNames {
    PageNames(
        req.cache(all_pages)
            .iter()
            .map(|page| page.name.clone())
            .collect::<Vec<_>>(),
    )
}

fn list_of_names(req: &Request) -> String {
    req.cache(page_names)
        .0
        .iter()
        .map(|name| format!("<li>{}</li>", name))
        .collect::<Vec<_>>()
        .join("\n")
}

fn list(req: Request) -> impl Responder {
    format!(
        "<html>
            <head><title>{title}</title></head>
            <body>
                <h1>{title}</h1>
                <h3>There are {page_count} pages:</h3>
                <ul>
                    {pages}
                </ul>
            </body>
        </html>",
        title = "List Pages",
        page_count = req.cache(all_pages).len(),
        pages = req.cache(list_of_names),
    )
}

fn main() {
    run!().unwrap();
}
