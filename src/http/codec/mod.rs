use std::io;
use std::str;
use tokio_core::io::{Codec, EasyBuf};

use http::{HttpRequest, HttpResponse};

mod parser;

pub struct HttpCodec;

impl Codec for HttpCodec {
    type In = HttpRequest;
    type Out = HttpResponse;

    fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<Self::In>> {
        parser::header(buf)
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
