#![doc(hidden)]

use crate::{request::Span, Error, Request};

/// Status of parsing an HTTP request. The request may have been only
/// partial, in which case the buffer is returned in `Partial` so we
/// can continue reading from the socket.
#[derive(Debug)]
pub enum Status {
    Complete(Request),
    Partial(Vec<u8>),
}

/// Parse a raw HTTP request into a Request struct.
pub fn parse(buffer: Vec<u8>) -> Result<Status, Error> {
    if buffer.len() < 10 {
        return Ok(Status::Partial(buffer));
    }

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

    let path_len = buffer[method_len + 1..].iter().position(|c| *c == b' ');
    if path_len.is_none() {
        return Ok(Status::Partial(buffer));
    }
    let path_len = path_len.unwrap();
    let pos = method_len + 1 + path_len + 1;
    if buffer.len() <= pos + 10 {
        return Ok(Status::Partial(buffer));
    }
    if &buffer[pos..pos + 8] != b"HTTP/1.1" {
        return Err(Error::ParseVersion);
    }
    let pos = pos + 8;
    if &buffer[pos..pos + 2] != b"\r\n" {
        return Err(Error::ExpectedCRLF);
    }

    let mut pos = pos + 2;
    let mut start = pos;
    let mut headers = Vec::with_capacity(16);
    let mut name = Span(0, 0);
    let mut saw_end = false;
    let mut parsing_key = true;

    let mut iter = buffer[pos..].iter();
    while let Some(c) = iter.next() {
        if parsing_key {
            match *c {
                b':' => {
                    name = Span(start, pos);
                    start = pos + 1;
                    parsing_key = false;
                }
                b'\r' | b'\n' | b' ' => return Err(Error::ParseHeaderName),
                _ => {}
            }
        } else if *c == b'\r' && buffer.get(pos + 1) == Some(&b'\n') {
            if name == Span(0, 0) {
                return Err(Error::ParseError);
            }

            headers.push((name, Span(start, pos)));
            name = Span(0, 0);
            iter.next();
            parsing_key = true;

            if buffer.get(pos + 2) == Some(&b'\r') && buffer.get(pos + 3) == Some(&b'\n') {
                pos += 4;
                saw_end = true;
                break;
            }

            start = pos + 2;
            pos += 1;
        }
        pos += 1;
    }

    // didn't receive full headers, abort
    if !saw_end {
        return Ok(Status::Partial(buffer));
    }

    let method = Span(0, method_len);
    let full_path = Span(method_len + 1, method_len + 1 + path_len);
    // path doesn't include ?query
    let path = if let Some(idx) = full_path.from_buf(&buffer).find('?') {
        Span(method_len + 1, method_len + 1 + idx)
    } else {
        full_path
    };
    let body = Span(pos, pos + buffer.len());

    Ok(Status::Complete(Request::new(
        method, full_path, path, headers, body, buffer,
    )))
}
