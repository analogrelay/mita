pub struct HttpRequest {
    pub method: String,
    pub uri: String,
    pub version: String
    // No headers/body yet
}

impl HttpRequest {
    pub fn new(method: String, uri: String, version: String) -> HttpRequest {
        HttpRequest {
            method: method,
            uri: uri,
            version: version
        }
    }
}
