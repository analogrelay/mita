use std::io;
use std::str;
use tokio_core::io::{Codec, EasyBuf};

use http::{HttpRequest, HttpResponse};

pub struct HttpCodec;

impl Codec for HttpCodec {
    type In = HttpRequest;
    type Out = HttpResponse;

    fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<Self::In>> {
        // We want to read the entire HTTP request header. This doesn't parse it, it just scans for
        // the end.
        if let Some(header) = read_request_header(buf) {

        } else {
            Ok(None)
        }

        //if let Some(request_line) = read_line(buf) {
            //// Parse it
            //if let Some((method, uri, version)) = parse_request_line(&mut request_line) {
                //// Ok, we have a request line. Now we need to parse headers
                //let headers = Vec::new();
                //while let Some(header) = read_header(buf) {
                //}
            //} else {
                //// TODO: Don't Panic!
                //panic!("invalid request line!");
            //}
        //} else {
            //// Not enough data to decode a frame
            //Ok(None)
        //}
    }

    fn encode(&mut self, msg: Self::Out, buf: &mut Vec<u8>) -> io::Result<()> {
        // Validate the response
        if msg.status_code > 999 {
            panic!("Invalid status code: {}. Maximum status code is 999", msg.status_code);
        }

        if msg.reason.len() == 0 {
            panic!("Reason string must be non-empty");
        }

        if msg.version.len() == 0 {
            panic!("HTTP Version string must be non-empty");
        }

        // Build the response line
        let s = format!("{} {} {}", msg.status_code, msg.reason, msg.version);

        // Append it to the buffer
        buf.extend_from_slice(s.as_bytes());
        Ok(())
    }
}

fn read_request_header(buf: &mut EasyBuf) -> Option<EasyBuf> {
}

fn find_end_of_frame(buf: &EasyBuf) -> Option<usize> {
    // Find the end-of-frame sequence '\r\n\r\n'
    let mut previous = [None, None, None];
    for (i, c) in buf.as_slice().iter().enumerate() {
        match (previous, *c) {
            ([Some(b'\r'), Some(b'\n'), Some(b'\r')], b'\n') => 
            _ => {}
        };
        previous[0] = previous[1];
        previous[1] = previous[2];
        previous[2] = Some(c);
    }
    None
}

fn read_header(buf: &mut EasyBuf) -> Option<EasyBuf> {
    if let Some(first_line) = read_line(buf) {
    } else {
        None
    }
}

fn parse_request_line(request_line: &mut EasyBuf) -> Option<(String, String, String)> {
    // Copy strings for now. We could look at storing EasyBuf slices in future.

    let method = match drain_to_separator(request_line, b' ') {
        Some(buf) => String::from_utf8(buf.as_slice().into()).unwrap(),
        _ => return None
    };

    let uri = match drain_to_separator(request_line, b' ') {
        Some(buf) => String::from_utf8(buf.as_slice().into()).unwrap(),
        _ => return None
    };

    // The rest of the buffer is the version
    let version = String::from_utf8(request_line.as_slice().into()).unwrap();

    Some((method, uri, version))
}

fn drain_to_separator(buf: &mut EasyBuf, sep: u8) -> Option<EasyBuf> {
    if let Some(i) = buf.as_slice().iter().position(|&b| b == sep) {
        // Drain the whole request line, including the newline. Then drain off everything
        // except the newline and let the buffer with the newline die
        Some(buf.drain_to(i + 1).drain_to(i))
    } else {
        // Didn't find the byte to split at
        None
    }
}

fn read_line(buf: &mut EasyBuf) -> Option<EasyBuf> {
    if let Some(i) = find_newline(buf) {
        // Drain the whole request line, including the newline. Then drain off everything
        // except the newline and let the buffer with the newline die
        let mut request_line = buf.drain_to(i + 1).drain_to(i - 1);
        Some(request_line)
    } else {
        None
    }
}
