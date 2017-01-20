extern crate futures;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;

/// Provides the protocol-level support for HTTP, primarily the `HttpRequest` and `HttpResponse`
/// types, along with the tokio `Codec` implementation for these
pub mod http;
