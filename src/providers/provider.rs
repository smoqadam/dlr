
pub trait Provider {
    fn file_info(&self, url: &str ) -> Result<super::FileInfo, super::AppError> ;
}