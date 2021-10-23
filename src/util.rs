const HTTP_DATE_FMT: &str = "%a, %d %b %Y %H:%M:%S";

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
    for _ in 0..512 {
        let next_pct = match inp.find('%') {
            Some(l) if l < if inp.len() > 1 { inp.len() - 2 } else { 0 } => l,
            Some(_) => return None,
            None => break,
        };
        let (push, pct_rest) = inp.split_at(next_pct);
        out.extend_from_slice(push.as_bytes());
        if pct_rest.is_char_boundary(3) {
            let (pct, rest) = pct_rest.split_at(3);
            inp = rest;
            let val = u8::from_str_radix(&pct[1..], 16).ok()?;
            out.push(val);
        }
    }
    out.extend_from_slice(inp.as_bytes());
    String::from_utf8(out).ok()
}

/// Content type for a file based on its extension.
/// https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Common_types
pub fn content_type(path: &str) -> &'static str {
    match path
        .split('.')
        .last()
        .unwrap_or("?")
        .to_lowercase()
        .as_ref()
    {
        "aac" => "audio/aac",
        "abw" => "application/x-abiword",
        "arc" => "application/x-freearc",
        "avi" => "video/x-msvideo",
        "azw" => "application/vnd.amazon.ebook",
        "bin" => "application/octet-stream",
        "bmp" => "image/bmp",
        "bz" => "application/x-bzip",
        "bz2" => "application/x-bzip2",
        "csh" => "application/x-csh",
        "css" => "text/css; charset=utf8",
        "csv" => "text/csv",
        "doc" => "application/msword",
        "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        "eot" => "application/vnd.ms-fontobject",
        "epub" => "application/epub+zip",
        "gz" => "gzip",
        "gif" => "image/gif",
        "htm" | "html" => "text/html; charset=utf8",
        "ico" => "image/vnd.microsoft.icon",
        "ics" => "text/calendar",
        "jar" => "application/java-archive",
        "jpeg" | "jpg" => "image/jpeg",
        "js" => "text/javascript",
        "json" => "application/json",
        "jsonld" => "application/ld+json",
        "mid" | "midi" => "audio/midi",
        "mjs" => "text/javascript",
        "mp3" => "audio/mpeg",
        "mpeg" => "video/mpeg",
        "mpkg" => "application/vnd.apple.installer+xml",
        "odp" => "application/vnd.oasis.opendocument.presentation",
        "ods" => "application/vnd.oasis.opendocument.spreadsheet",
        "odt" => "application/vnd.oasis.opendocument.text",
        "oga" => "audio/ogg",
        "ogv" => "video/ogg",
        "ogx" => "application/ogg",
        "opus" => "audio/opus",
        "otf" => "font/otf",
        "png" => "image/png",
        "pdf" => "application/pdf",
        "php" => "application/x-httpd-php",
        "ppt" => "application/vnd.ms-powerpoint",
        "pptx" => "application/vnd.openxmlformats-officedocument.presentationml.presentation",
        "rar" => "application/vnd.rar",
        "rtf" => "application/rtf",
        "sh" => "application/x-sh",
        "svg" => "image/svg+xml",
        "swf" => "application/x-shockwave-flash",
        "tar" => "application/x-tar",
        "tif" | "tiff" => "image/tiff",
        "ts" => "video/mp2t",
        "ttf" => "font/ttf",
        "txt" => "text/plain; charset=utf8",
        "vsd" => "application/vnd.visio",
        "wav" => "audio/wav",
        "weba" => "audio/webm",
        "webm" => "video/webm",
        "webp" => "image/webp",
        "woff" => "font/woff",
        "woff2" => "font/woff2",
        "xhtml" => "application/xhtml+xml",
        "xls" => "application/vnd.ms-excel",
        "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "xul" => "application/vnd.mozilla.xul+xml",
        "xml" => "application/xml",
        "zip" => "application/zip",
        "7z" => "application/x-7z-compressed",
        _ => "text/plain; charset=utf8",
    }
}
