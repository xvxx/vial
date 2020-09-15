use crate::{request::Span, Error, Request};

/// Status of parsing an HTTP request. The request may have been only
/// partial, in which case the buffer is returned in `Partial` so we
/// can continue reading from the socket.
#[derive(Debug)]
pub enum Status {
    Complete(Request),
    Partial(Vec<u8>),
}

/// Total size limit for all headers combined.
const MAX_HEADER_SIZE: usize = 8192;

/// Parse a raw HTTP request into a Request struct.
pub fn parse(mut buffer: Vec<u8>) -> Result<Status, Error> {
    let mut pos = 0;

    // clear preceding \n or \r
    while !buffer.is_empty() && buffer[0].is_ascii_whitespace() {
        buffer.remove(0);
    }

    macro_rules! need {
        ($size:expr) => {
            if buffer[pos..].len() < $size {
                return Ok(Status::Partial(buffer));
            } else {
                true
            }
        };
    }

    need!(10);

    // Parse method: GET / HTTP/1.1
    let method_len = {
        match &buffer[0..3] {
            b"GET" | b"PUT" => 3,
            b"HEA" | b"POS" => match &buffer[0..4] {
                b"HEAD" | b"POST" => 4,
                _ => 0,
            },
            b"PAT" | b"TRA" => match &buffer[0..5] {
                b"PATCH" | b"TRACE" => 5,
                _ => 0,
            },
            b"DEL" => {
                if &buffer[0..6] == b"DELETE" {
                    6
                } else {
                    0
                }
            }
            b"CON" | b"OPT" => match &buffer[0..7] {
                b"CONNECT" | b"OPTIONS" => 7,
                _ => 0,
            },
            _ => 0,
        }
    };

    if method_len == 0 {
        return Err(Error::UnknownHTTPMethod("?".into()));
    }

    if buffer[method_len] != b' ' {
        return Err(Error::ParseError);
    }

    // Parse path: GET / HTTP/1.1
    let path_len = buffer[method_len + 1..].iter().position(|c| *c == b' ');
    if path_len.is_none() {
        return Ok(Status::Partial(buffer));
    }
    let path_len = path_len.unwrap();
    pos = method_len + 1 + path_len + 1;

    // Parse version: GET / HTTP/1.1
    for c in b"HTTP/1.1" {
        if buffer.len() <= pos {
            return Ok(Status::Partial(buffer));
        } else if buffer[pos] == *c {
            pos += 1;
        } else {
            return Err(Error::ParseVersion);
        }
    }

    // Parse first line break
    if need!(1) && buffer[pos] == b'\n' {
        pos += 1;
    } else if need!(2) && &buffer[pos..pos + 2] == b"\r\n" {
        pos += 2;
    } else {
        return Err(Error::ExpectedCRLF);
    }

    // End here if there are no headers.
    if (need!(1) && buffer[pos] == b'\n') || (need!(2) && &buffer[pos..pos + 2] == b"\r\n") {
        let method = Span(0, method_len);
        let path = Span(method_len + 1, method_len + 1 + path_len);

        return Ok(Status::Complete(Request::new(
            method,
            path,
            Vec::new(),
            Span::new(),
            buffer,
        )));
    }

    // Parse headers
    let mut start = pos;
    let mut headers = Vec::with_capacity(16);
    let mut name = Span::new();
    let mut saw_end = false;
    let mut parsing_key = true;
    let mut content_length = 0;
    let mut len = 0; // header length

    while let Some(c) = buffer.get(pos) {
        if parsing_key {
            match *c {
                b':' => {
                    name = Span(start, pos);
                    // skip leading whitespace on value
                    loop {
                        match buffer.get(pos + 1) {
                            Some(&b' ') | Some(&b'\t') => pos += 1,
                            _ => break,
                        }
                    }
                    parsing_key = false;
                    start = pos + 1;
                }
                b'\r' | b'\n' | b' ' | b'\t' => return Err(Error::ParseHeaderName),
                _ => {}
            }
        } else {
            if *c == b'\n' || (*c == b'\r' && buffer.get(pos + 1) == Some(&b'\n')) {
                if name.is_empty() {
                    return Err(Error::ParseError);
                }

                let value = Span(start, pos);
                headers.push((name, value));
                if name.from_buf(&buffer).to_ascii_lowercase() == "content-length" {
                    content_length = value.from_buf(&buffer).parse().unwrap_or(0);
                }

                name = Span::new();
                parsing_key = true;

                // skip \r\n or \n
                pos += if *c == b'\n' { 1 } else { 2 };

                if let Some(next) = buffer.get(pos) {
                    match *next {
                        b'\n' => {
                            pos += 1;
                            saw_end = true;
                            break;
                        }
                        b'\r' => {
                            if buffer.get(pos + 1) != Some(&b'\n') {
                                return Ok(Status::Partial(buffer));
                            }
                            pos += 2;
                            saw_end = true;
                            break;
                        }
                        _ => {}
                    }
                }

                start = pos;
                continue;
            }
        }
        len += 1;
        if len > MAX_HEADER_SIZE {
            return Err(Error::ParseHeaderValue);
        }
        pos += 1;
    }

    // didn't receive full headers, abort
    if !saw_end {
        return Ok(Status::Partial(buffer));
    }

    let method = Span(0, method_len);
    let path = Span(method_len + 1, method_len + 1 + path_len);
    let body = if content_length > 0 {
        Span(pos, pos + buffer.len())
    } else {
        Span::new()
    };

    Ok(Status::Complete(Request::new(
        method, path, headers, body, buffer,
    )))
}
