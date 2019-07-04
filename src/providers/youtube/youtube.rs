extern crate regex;

use super::video_info;
use crate::providers::{FileInfo, AppError};
use crate::request::Request;
use crate::Provider;
use regex::Regex;
use std::io::{Read};

pub struct Youtube;

impl Youtube {
    /// get a Youtube url and retrive direct video url and video title
    pub fn get_video_info(&self, url: &str) -> Result<FileInfo, AppError> {
        let regex = Regex::new(r"^.*(?:(?:youtu\.be/|v/|vi/|u/w/|embed/)|(?:(?:watch)?\?v(?:i)?=|\&v(?:i)?=))([^#\&\?]*).*").unwrap();
        if !regex.is_match(url) {
            return Err(AppError);
        }

        let video_info_url = video_info::video_info_url_from_url(url).unwrap();// format!("https://youtube.com/get_video_info?video_id={}", vid_url);
        
        let mut response = Request::new(&video_info_url);

        let mut response_str = String::new();
        response.read_to_string(&mut response_str).unwrap();

        let info = match video_info::VideoInfo::parse(&response_str) {
            Ok(info) => info,
            Err(_) => {
                // TODO: better error handling
                println!("Youtube video not found: {}", url);
                std::process::exit(1);
            }
        };

        let stream = self.get_stream(&info.streams);
        let file_name = match stream.extension() {
            Some(ext) => format!("{}.{}", info.title, ext),
            None => info.title,
        };
        let direct_url = stream.url.clone();
        Ok(FileInfo {
            file_name, direct_url, orig_url: url.to_string(),
        })
    }


    /// show a simple menu to choose option
    /// default option is 0
    fn get_stream<'a>(&self, streams: &'a Vec<video_info::Stream>) -> &'a  video_info::Stream {
        for (i, stream) in streams.iter().enumerate() {
            println!("{}- {} {}", i, stream.quality, stream.stream_type);
        }

        println!("Choose quality (default: 0): ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap_or(0);
        let mut input = input.trim().parse().unwrap_or(0);
        if input > streams.len() {
            println!("Quality {} not found, go with default quality", input);  
            input = 0;
        }
        let stream = streams.get(input).unwrap();
        stream
    }
}

impl Provider for Youtube {
    fn file_info(&self, url: &str ) -> Result<FileInfo, AppError>  {
        self.get_video_info(url)
    }
}
