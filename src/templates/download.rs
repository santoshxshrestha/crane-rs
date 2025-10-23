use crate::FileInfo;
use crate::PathBuf;
use askama::Template;
use std::f64;
use std::time::Duration;
use std::time::SystemTime;

#[derive(Template)]
#[template(path = "download.html")]
pub struct DownloadTemplate {
    content: String,
    files: Vec<FileInfo>,
}

impl DownloadTemplate {
    pub fn new(files: Vec<PathBuf>, content: String) -> Self {
        DownloadTemplate {
            files: files
                .into_iter()
                .map(|path| {
                    let name = path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();
                    let metadata = path.metadata();

                    match metadata {
                        Ok(ref m) => {
                            let size = m.len() as f64 / 1_000_000_f64;
                            let string_size = format!("{size:.2}");

                            let created = m.created().unwrap_or(SystemTime::now());
                            let now = SystemTime::now();
                            let duration = now
                                .duration_since(created)
                                .unwrap_or(Duration::from_secs(0))
                                .as_secs();
                            let time_ago = if duration < 60 {
                                "just now".to_string()
                            } else if duration < 3600 {
                                format!("{:.2} minutes ago", duration as f64 / 60_f64)
                            } else if duration < 86400 {
                                format!("{:.2} hours ago", duration as f64 / 3600_f64)
                            } else {
                                format!("{:.2} days ago", duration as f64 / 86400_f64)
                            };
                            let path = path.to_string_lossy().to_string();
                            FileInfo::new(name, path, string_size, time_ago)
                        }
                        Err(_) => {
                            let path = path.to_string_lossy().to_string();
                            FileInfo::new(name, path, "0.00".to_string(), "unknown".to_string())
                        }
                    }
                })
                .collect(),
            content,
        }
    }
}
