extern crate reqwest;

pub struct Request;
impl Request {
    pub fn new(url: &str) -> reqwest::Response {
        let client = reqwest::Client::new();
        client.get(url).send().unwrap()
    }
}

pub struct Error;
impl Error {}
