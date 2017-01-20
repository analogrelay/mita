pub struct HttpResponse {
    pub status_code: usize,
    pub reason: String,
    pub version: String
    // No headers/body yet
}

impl HttpResponse {
    pub fn new(status_code: usize, reason: String, version: String) -> HttpResponse {
        HttpResponse {
            status_code: status_code,
            reason: reason,
            version: version
        }
    }
}
