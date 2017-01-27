use std::io;
use tokio_core::io::EasyBuf;
use http::HttpRequest;

// Primary Parsing functions

pub fn header(buf: &mut EasyBuf) -> io::Result<Option<HttpRequest>> {
    // We want to read the entire HTTP request header. This doesn't parse it, it just scans for
    // the end.
    if let Some(_header) = read_request_header(buf) {
        unimplemented!()
    } else {
        Ok(None)
    }
}

// Helper functions

fn find_end_of_headers(buf: &EasyBuf) -> Option<usize> {
    // Find the end-of-frame sequence '\r\n\r\n'
    let mut previous = [None, None, None];
    for (i, c) in buf.as_slice().iter().enumerate() {
        if (previous[0], previous[1], previous[2], *c) == (Some(b'\r'), Some(b'\n'), Some(b'\r'), b'\n') {
            return Some(i - 3)
        }
        previous[0] = previous[1];
        previous[1] = previous[2];
        previous[2] = Some(*c);
    }
    None
}

fn read_request_header(buf: &mut EasyBuf) -> Option<EasyBuf> {
    if let Some(end) = find_end_of_headers(buf) {
        let header = buf.drain_to(end);
        buf.drain_to(4);
        Some(header)
    } else {
        None
    }
}

#[allow(dead_code)]
fn drain_to(buf: &mut EasyBuf, sep: u8) -> Option<EasyBuf> {
    if let Some(i) = buf.as_slice().iter().position(|&b| b == sep) {
        // Drain the whole request line, including the newline. Then drain off everything
        // except the newline and let the buffer with the newline die
        Some(buf.drain_to(i + 1).drain_to(i))
    } else {
        // Didn't find the byte to split at
        None
    }
}

#[cfg(test)]
mod test {
    use std::str;

    use super::*;
    use tokio_core::io::EasyBuf;

    macro_rules! easy_buf {
        ($val:expr) => (EasyBuf::from($val.to_vec()))
    }

    macro_rules! easy_buf_to_str {
        ($buf:expr) => (str::from_utf8($buf.as_slice()).unwrap())
    }

    #[test]
    pub fn find_end_of_headers_can_find_the_end_of_the_frame() {
        assert_eq!(Some(10), find_end_of_headers(&easy_buf!(b"A\r\n\rB\r\n\nC\r\r\n\r\n")));
    }

    #[test]
    pub fn find_end_of_headers_works_with_buffer_that_is_too_small() {
        assert_eq!(None, find_end_of_headers(&easy_buf!(b"A")));
    }

    #[test]
    pub fn find_end_of_headers_returns_none_if_no_end_of_frame() {
        assert_eq!(None, find_end_of_headers(&easy_buf!(b"A\r\nB\r\n\r")));
    }

    #[test]
    pub fn find_end_of_headers_works_with_empty_frame() {
        assert_eq!(Some(0), find_end_of_headers(&easy_buf!(b"\r\n\r\n")));
    }

    #[test]
    pub fn read_request_header_returns_just_the_header_with_no_crlf() {
        let mut buf = easy_buf!(b"A\r\n\rB\r\n\nC\r\r\n\r\nNext");
        let header = read_request_header(&mut buf);
        assert_eq!("A\r\n\rB\r\n\nC\r", easy_buf_to_str!(&header.unwrap()));
        assert_eq!("Next", easy_buf_to_str!(&buf));
    }

    #[test]
    pub fn read_request_header_returns_none_if_no_complete_header_available() {
        assert!(read_request_header(&mut easy_buf!(b"A\r\nB\r\nC\r\n")).is_none());
    }

    #[test]
    pub fn drain_to_returns_none_if_separator_not_present() {
        assert!(drain_to(&mut easy_buf!(b"ABCDEF"), b' ').is_none());
    }

    #[test]
    pub fn drain_to_leaves_buffer_unmodified_if_separator_not_present() {
        let mut buf = easy_buf!(b"ABCDEF");
        drain_to(&mut buf, b' ');
        assert_eq!("ABCDEF", easy_buf_to_str!(&buf));
    }

    #[test]
    pub fn drain_to_drains_to_first_instance_of_separator() {
        let mut buf = easy_buf!(b"Aaa Bbb Ccc Ddd Eee Fff");
        let first = drain_to(&mut buf, b' ').unwrap();
        assert_eq!("Aaa", easy_buf_to_str!(&first));
        assert_eq!("Bbb Ccc Ddd Eee Fff", easy_buf_to_str!(&buf));
    }
}
