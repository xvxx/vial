use percent_encoding::percent_decode;

const HTTP_DATE_FMT: &str = "%a, %d %b %Y %H:%M:%S %Z";

/// Content type for a file based on its extension.
pub fn content_type(path: &str) -> &'static str {
    match path.split('.').last().unwrap_or("?") {
        "gif" => "image/gif",
        "jpg" => "image/jpeg",
        "jpeg" => "image/jpeg",
        "png" => "image/png",
        "pdf" => "application/pdf",
        "css" => "text/css; charset=utf8",
        "htm" => "text/html; charset=utf8",
        "html" => "text/html; charset=utf8",
        "txt" => "text/plain; charset=utf8",
        _ => "text/plain; charset=utf8",
    }
}

/// Does what it says.
pub fn decode_form_value(post: &str) -> String {
    let cleaned = post.replace('+', " ").replace('\r', "");
    percent_decode(cleaned.as_bytes())
        .decode_utf8_lossy()
        .to_string()
}

/// Current date in HTTP format.
pub fn http_current_date() -> String {
    let now = libc_strftime::epoch();
    libc_strftime::strftime_gmt(HTTP_DATE_FMT, now)
}
