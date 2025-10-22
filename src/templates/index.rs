use askama::Template;
#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    content: String,
}
impl IndexTemplate {
    pub fn new(content: String) -> Self {
        IndexTemplate { content }
    }
}
