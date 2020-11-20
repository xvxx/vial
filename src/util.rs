const HTTP_DATE_FMT: &str = "%a, %d %b %Y %H:%M:%S";

/// Content type for a file based on its extension.
pub fn content_type(path: &str) -> &'static str {
    match path
        .split('.')
        .last()
        .unwrap_or("?")
        .to_lowercase()
        .as_ref()
    {
        "gif" => "image/gif",
        "jpg" => "image/jpeg",
        "jpeg" => "image/jpeg",
        "png" => "image/png",
        "svg" => "image/svg+xml",
        "pdf" => "application/pdf",
        "css" => "text/css; charset=utf8",
        "htm" => "text/html; charset=utf8",
        "html" => "text/html; charset=utf8",
        "txt" => "text/plain; charset=utf8",
        _ => "text/plain; charset=utf8",
    }
}

/// Size of a file on disk. 0 if it doesn't exist.
pub fn file_size(path: &str) -> usize {
    std::fs::File::open(path)
        .map(|f| f.metadata().map(|m| m.len()).unwrap_or(0))
        .unwrap_or(0) as usize
}

/// Does what it says.
pub fn decode_form_value(post: &str) -> String {
    let cleaned = post.replace('+', " ").replace('\r', "");
    percent_decode(&cleaned).unwrap_or_else(|| "".into())
}

/// Current date in HTTP format.
pub fn http_current_date() -> String {
    let now = libc_strftime::epoch();
    libc_strftime::strftime_gmt(HTTP_DATE_FMT, now) + " GMT"
}

/// Mutably borrowed from the zero dependency httpserv project.
/// https://github.com/nic-hartley/httpserv/blob/585c020/src/http.rs
pub fn percent_decode(mut inp: &str) -> Option<String> {
    let mut out = Vec::new();
    loop {
        let next_pct = match inp.find('%') {
            Some(l) if l < inp.len() - 2 => l,
            Some(_) => return None,
            None => break,
        };
        let (push, pct_rest) = inp.split_at(next_pct);
        out.extend_from_slice(push.as_bytes());
        let (pct, rest) = pct_rest.split_at(3);
        inp = rest;
        let val = u8::from_str_radix(&pct[1..], 16).ok()?;
        out.push(val);
    }
    out.extend_from_slice(inp.as_bytes());
    String::from_utf8(out).ok()
}
