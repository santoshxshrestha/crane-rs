use crate::FileInfo;
use crate::PathBuf;
use askama::Template;
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

                    let size = path
                        .metadata()
                        .map(|m| m.len() as f64 / 1_000_000_f64)
                        .unwrap_or(0 as f64);
                    let size_string = format!("{size:.2}");
                    let created = path
                        .metadata()
                        .and_then(|m| m.created())
                        .unwrap_or(SystemTime::now());
                    let now = SystemTime::now();
                    let duration = now
                        .duration_since(created)
                        .unwrap_or(Duration::from_secs(0));
                    let minutes_ago = duration.as_secs() / 60;
                    let time_ago = if minutes_ago == 0 {
                        "just now".to_string()
                    } else if minutes_ago < 60 {
                        format!("{minutes_ago} minutes ago")
                    } else if minutes_ago > 60 && minutes_ago < 1440 {
                        format!("{:.2} hours ago", minutes_ago / 60)
                    } else {
                        format!("{:.2} days ago", minutes_ago / 1440)
                    };
                    let file = path.to_string_lossy().to_string();
                    FileInfo::new(name, file, size_string, time_ago)
                })
                .collect(),
            content,
        }
    }
}
