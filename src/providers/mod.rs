pub mod provider;
pub mod providers;
pub mod youtube;
pub mod direct;

pub use providers::Providers;
pub use provider::{Provider};
pub use youtube::Youtube;
pub use direct::Direct;


use std::fmt;

// Custom error type; can be any type which defined in the current crate
pub struct AppError;

// Implement std::fmt::Display for AppError
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An Error Occurred, Please Try Again!") // user-facing output
    }
}

// Implement std::fmt::Debug for AppError
impl fmt::Debug for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}


pub struct FileInfo {
        file_name: String,
        orig_url: String,
        direct_url: String,
}

impl FileInfo {
    pub fn filename(&self) -> &str {
        self.file_name.as_str()
    }

    pub fn orig_url(&self) -> &str {
        self.orig_url.as_str()
    }

    pub fn direct_url(&self) -> &str {
        self.direct_url.as_str()
    }
}