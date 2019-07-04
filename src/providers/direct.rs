extern crate regex;
use super::{AppError, FileInfo};
use crate::request::Request;
use super::{Provider};
use regex::Regex;

pub struct Direct;

impl Provider for Direct {
    fn file_info(&self, url: &str ) -> Result<FileInfo, AppError> {
        self.get_file_info(url)   
    }
}

impl Direct {
    pub fn get_file_info(&self, url: &str) -> Result<FileInfo, AppError> {
        let regex = Regex::new(r"^(http|https)://").unwrap();
        if !regex.is_match(url) {
            return Err(AppError);
        }
        
        let response = Request::new(url);
        let file_name = {
            response
                .url()
                .path_segments()
                .and_then(|segments| segments.last())
                .and_then(|name| if name.is_empty() { None } else { Some(name) })
                .unwrap()
        };

        Ok(FileInfo {
            file_name: file_name.to_string(),
            direct_url: url.to_string(),
            orig_url: url.to_string(),
        })
    }
}