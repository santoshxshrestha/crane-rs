use askama::Template;
#[derive(Template)]
#[template(path = "upload.html")]
pub struct UploadTemplate {
    content: String,
}
impl UploadTemplate {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}
