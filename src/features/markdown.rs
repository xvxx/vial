use {crate::Response, pulldown_cmark as markdown, std::fmt};

impl Response {
    pub fn as_markdown(mut self) -> Self {
        let mut options = markdown::Options::empty();
        options.insert(markdown::Options::ENABLE_TABLES);
        options.insert(markdown::Options::ENABLE_FOOTNOTES);
        options.insert(markdown::Options::ENABLE_STRIKETHROUGH);
        options.insert(markdown::Options::ENABLE_TASKLISTS);

        let parser = markdown::Parser::new_ext(&self.body, options).map(|event| match event {
            markdown::Event::Text(text) => {
                if text.contains("http://") || text.contains("https://") {
                    let linked = autolink::auto_link(&text, &[]);
                    if linked.len() == text.len() {
                        markdown::Event::Text(text.into())
                    } else {
                        markdown::Event::Html(linked.into())
                    }
                } else {
                    markdown::Event::Text(text.into())
                }
            }
            _ => event,
        });

        let mut html_output = String::with_capacity(self.body.len() * 3 / 2);
        markdown::html::push_html(&mut html_output, parser);
        self.body = html_output;
        self
    }
}
