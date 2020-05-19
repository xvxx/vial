use percent_encoding::percent_decode;

/// Content type for a file based on its extension.
pub fn get_content_type(path: &str) -> &'static str {
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
    percent_decode(post.as_bytes())
        .decode_utf8_lossy()
        .replace('+', " ")
        .replace('\r', "")
}
