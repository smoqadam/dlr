
extern crate reqwest;
use super::request::Request;
use std::io::Read;

pub struct Downloader {
    response: reqwest::Response,
}

impl Downloader {
    pub fn new(url: &str) -> Downloader {
        let response = Request::new(url);
        Downloader { response }
    }

    pub fn file_size(&self) -> u64 {
        self.response.content_length().unwrap()
    }

    pub fn dl(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        self.response.read(buf)
    }
}
