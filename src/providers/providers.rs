use super::{Provider};

pub struct Providers {
    providers: Vec<Box<Provider>>,
}

impl Providers {

    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
        }
    }
    pub fn fileinfo(&self, url: &str) -> Option<super::FileInfo> {
        for p in &self.providers {
                match p.file_info(url) {
                    Ok(pr) => return Some(pr),
                    Err(_) => (),
                }
        }
        None
    }

    pub fn register<P: Provider + 'static>(&mut self, p: P) -> &mut Self {
        self.providers.push(Box::new(p));
        self
    }
}