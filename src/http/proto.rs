use std::io;
use tokio_core::io::{Io, Framed};
use tokio_proto::pipeline::ServerProto;

use http::{HttpCodec, HttpRequest, HttpResponse};

pub struct HttpProto;

impl<T: Io + 'static> ServerProto<T> for HttpProto {
    type Request = HttpRequest;
    type Response = HttpResponse;
    type Transport = Framed<T, HttpCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(HttpCodec))
    }
}
