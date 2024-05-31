use std::collections::HashMap;

use rust_extensions::date_time::DateTimeAsMicroseconds;

pub struct HttpResponseCache {
    pub until: DateTimeAsMicroseconds,
    pub content: Vec<u8>,
}

impl HttpResponseCache {
    pub fn new(content: Vec<u8>) -> Self {
        let mut until = DateTimeAsMicroseconds::now();
        until.add_seconds(30);
        Self { until, content }
    }
}

pub struct HttpGetRequestsCache {
    pub cache: HashMap<String, HttpResponseCache>,
}

impl HttpGetRequestsCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn set_cached_response(&mut self, url: String, response: HttpResponseCache) {
        self.cache.insert(url, response);
    }

    pub fn get_cached_response(&self, url: &str) -> Option<Vec<u8>> {
        let result = self.cache.get(url)?;

        let now = DateTimeAsMicroseconds::now();

        if result.until.unix_microseconds < now.unix_microseconds {
            return None;
        }

        Some(result.content.clone())
    }
}
