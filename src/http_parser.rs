use crate::error::Error::ConnectionClosed;
use crate::{request::Span, Error, Request};
use std::panic;

/// Total size limit for all headers combined.
const MAX_HEADER_SIZE: usize = 8192;

/// Parse a raw HTTP request into a Request struct.
pub fn parse(buffer: Vec<u8>) -> Result<Request, Error> {
    let mut pos = 0;

    macro_rules! peek {
        ($size:expr) => {
            if buffer[pos..].len() < $size {
                return Err(Error::ConnectionClosed);
            } else {
                true
            }
        };
    }

    macro_rules! consume_whitespace {
        () => {
            while peek!(1) && buffer[pos].is_ascii_whitespace() {
                pos += 1;
            }
        };
    }

    macro_rules! consume_whitespace_to_eol {
        () => {
            while buffer.get(pos).is_some() && buffer.get(pos).unwrap().is_ascii_whitespace()
                && (buffer[pos] != b'\r' && buffer[pos] != b'\n')
            {
                if !peek!(1) {
                    break;
                }
                pos += 1;
            }
        };
    }

    macro_rules! consume {
        ($word:expr) => {
            consume!($word, false)
        };
        ($word:expr, $error:expr) => {
            let mut found = true;
            for c in $word.bytes() {
                if buffer.len() <= pos {
                    return Err(Error::ConnectionClosed);
                } else if buffer[pos] == c {
                    pos += 1;
                } else {
                    found = false;
                    break;
                }
            }
            if found {
                true
            } else {
                $error
            }
        };
    }

    macro_rules! consume_eol {
        () => {
            consume_eol!(false)
        };
        ($error:expr) => {
            if (peek!(1) && buffer[pos] == b'\n') {
                pos += 1;
                true
            } else if (peek!(2) && &buffer[pos..pos + 2] == b"\r\n") {
                pos += 2;
                true
            } else {
                $error
            }
        };
    }

    consume_whitespace!();
    let method = parse_method(&buffer, &mut pos)?;

    consume!(" ", return Err(Error::ParseError));
    let path = parse_path(&buffer, &mut pos)?;

    consume!(" ", return Err(Error::ParseError));
    consume!("HTTP/1.1", return Err(Error::ParseVersion));
    consume_eol!(return Err(Error::ExpectedCRLF));

    // Expecting Headers but if there's another EOL we're done and we can just return the struct
    if consume_eol!() {
        return Ok(Request::new(method, path, Vec::new(), Span::new(), buffer));
    }

    // Parse headers
    let start = pos;
    let mut headers = Vec::with_capacity(16);
    let mut content_length = 0;

    loop {
        let name = parse_header_name(&buffer, &mut pos)?;
        consume!(":");
        consume_whitespace_to_eol!();
        let value = parse_header_value(&buffer, &mut pos)?;

        if name.from_buf(&buffer).to_ascii_lowercase() == "content-length" {
            content_length = value.from_buf(&buffer).parse().unwrap_or(0);
        }
        headers.push((name, value));

        consume_eol!(return Err(Error::ConnectionClosed));

        if consume_eol!() {
            break;
        }
        if pos - start > MAX_HEADER_SIZE {
            return Err(Error::ParseHeaderValue);
        }
    }

    let body = if content_length > 0 {
        Span(pos, pos + content_length)
    } else {
        Span::new()
    };

    if body.1 > buffer.len() {
        return Err(ConnectionClosed);
    }

    Ok(Request::new(method, path, headers, body, buffer))
}

fn parse_method(buffer: &Vec<u8>, pos: &mut usize) -> Result<Span, Error> {
    let start = *pos;
    if let Some(bytes) = buffer.get(start..start + 3) {
        let size = match bytes {
            b"GET" | b"PUT" => 3,
            b"HEA" | b"POS" => match buffer.get(start..start + 4) {
                Some(bytes) => match bytes {
                    b"HEAD" | b"POST" => 4,
                    _ => 0,
                },
                None => return Err(Error::ParseError),
            },
            b"PAT" | b"TRA" => match buffer.get(start..start + 5) {
                Some(bytes) => match bytes {
                    b"PATCH" | b"TRACE" => 5,
                    _ => 0,
                },
                None => return Err(Error::ParseError),
            },
            b"DEL" => match buffer.get(0..6) {
                Some(buffer) => {
                    if &buffer[0..6] == b"DELETE" {
                        6
                    } else {
                        0
                    }
                }
                None => return Err(Error::ParseError),
            },
            b"CON" | b"OPT" => match buffer.get(start..start + 7) {
                Some(bytes) => match bytes {
                    b"CONNECT" | b"OPTIONS" => 7,
                    _ => 0,
                },
                None => return Err(Error::ParseError),
            },
            _ => 0,
        };
        if size == 0 {
            return Err(Error::UnknownHTTPMethod("?".into()));
        } else {
            *pos += size;
            return Ok(Span(start, start + size));
        }
    }
    return Err(Error::ParseError);
}

fn parse_path(buffer: &Vec<u8>, pos: &mut usize) -> Result<Span, Error> {
    let start = *pos;
    let end = buffer[start..].iter().position(|c| *c == b' ');
    let end = match end {
        Some(number) => number,
        None => 0,
    };
    if end == 0 {
        return Err(Error::ParsePath);
    };
    *pos += end;
    Ok(Span(start, start + end))
}

fn parse_header_name(buffer: &Vec<u8>, pos: &mut usize) -> Result<Span, Error> {
    let start = *pos;
    loop {
        match buffer.get(*pos) {
            Some(bytes) => match bytes {
                b':' => break,
                b'\r' | b'\n' | b' ' | b'\t' => return Err(Error::ParseHeaderName),
                _ => {
                    *pos += 1;
                    if *pos == buffer.len() {
                        return Err(Error::ParseHeaderName);
                    }
                }
            },
            None => return Err(Error::ParseError),
        }
    }
    let end = *pos;
    Ok(Span(start, end))
}

fn parse_header_value(buffer: &Vec<u8>, pos: &mut usize) -> Result<Span, Error> {
    let start = *pos;
    while *pos < buffer.len() && (buffer[*pos] != b'\r' && buffer[*pos] != b'\n') {
        *pos += 1;
    }
    let end = *pos;
    Ok(Span(start, end))
}