use crate::MultipartForm;
use crate::TempFile;

pub struct FileInfo {
    pub name: String,
    pub file: String,
    pub size: String,
    pub time: String,
}
impl FileInfo {
    pub fn new(name: String, file: String, size: String, time: String) -> Self {
        Self {
            name,
            file,
            size,
            time,
        }
    }
}

#[derive(MultipartForm, Debug)]
pub struct UploadForm {
    #[multipart(limit = "1000 MiB")]
    pub file: TempFile,
}
