use crate::MultipartForm;
use crate::TempFile;

pub struct FileInfo {
    pub name: String,
    pub file: String,
}
impl FileInfo {
    pub fn new(name: String, file: String) -> Self {
        Self { name, file }
    }
}

#[derive(MultipartForm, Debug)]
pub struct UploadForm {
    pub file: TempFile,
}
