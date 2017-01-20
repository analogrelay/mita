pub struct HttpRequest {
    pub verb: String,
    pub uri: String,
    pub version: String
    // No headers/body yet
}

impl HttpRequest {
    pub fn new(verb: String, uri: String, version: String) -> HttpRequest {
        HttpRequest {
            verb: verb,
            uri: uri,
            version: version
        }
    }
}
