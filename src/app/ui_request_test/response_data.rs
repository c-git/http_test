use std::fmt::Debug;

use reqwest_cross::reqwest::StatusCode;

use super::cookies::Cookie;

pub struct ResponseData {
    pub url: String,
    pub text: String,
    pub status: StatusCode,
    pub size_kb: Option<f32>,
    pub headers: Vec<(String, String)>,
    pub cookies: Vec<Cookie>,
}

impl Debug for ResponseData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResponseData")
            .field("url", &self.url)
            .field("text_len", &self.text.len())
            .field("status", &self.status)
            .field("size_kb", &self.size_kb)
            .field("headers", &self.headers)
            .field("cookies", &self.cookies)
            .finish()
    }
}
