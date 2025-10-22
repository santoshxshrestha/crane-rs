use crate::FileInfo;
use crate::PathBuf;
use askama::Template;
use std::time::Duration;
use std::time::SystemTime;

#[derive(Template)]
#[template(
    ext = "html",
    source = r#"
<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/7.0.1/css/all.min.css" integrity="sha512-2SwdPD6INVrV/lHTZbO2nodKhrnDdJK9/kg2XD1r9uGqPo1cUbujc+IYdlYdEErWNu69gVcYgdxlmVmzTWnetw==" crossorigin="anonymous" referrerpolicy="no-referrer" />
    <title>crane.rs - download</title>
    <style>

body {
  background: #181825;
  color: #cdd6f4;
  font-family: 'Inter', 'Roboto', Arial, sans-serif;
  margin: 0;
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
}
.container {
  background: #232336;
  border-radius: 12px;
  border: 1px solid #313244;
  padding: 2rem 1.5rem;
  max-width: 350px;
  width: 100%;
  text-align: center;
}
.title {
  font-size: 1.7rem;
  font-weight: 600;
  margin-bottom: 1.2rem;
  color: #b4befe;
}
.nav-links {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  margin-top: 1.2rem;
}
.nav-link {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background: #313244;
  color: #cdd6f4;
  border: none;
  border-radius: 8px;
  padding: 1.2rem 2rem;
  font-size: 1.1rem;
  text-decoration: none;
  font-weight: 500;
  transition: background 0.2s;
  gap: 2rem;
  width: 100%;
  box-sizing: border-box;
  flex-wrap: nowrap;
  overflow: visible;
}
.file-name {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-weight: 600;
}
.file-meta {
  display: flex;
  gap: 1.2rem;
  font-size: 0.95rem;
  color: #b4befe;
  min-width: 0;
  flex-shrink: 0;
}
.file-size, .file-time {
  display: flex;
  align-items: center;
  gap: 0.3rem;
  min-width: 0;
}
.container {
  background: #232336;
  border-radius: 12px;
  border: 1px solid #313244;
  padding: 2.5rem 2.5rem;
  max-width: 700px;
  width: 100%;
  text-align: center;
  box-sizing: border-box;
}
.nav-links {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  margin-top: 1.2rem;
  width: 100%;
  box-sizing: border-box;
}
@media (max-width: 700px) {
  .container {
    padding: 1.2rem 0.5rem;
    max-width: 98vw;
  }
  .title {
    font-size: 1.2rem;
  }
  .nav-link {
    font-size: 1rem;
    padding: 0.8rem 0.5rem;
    flex-direction: column;
    align-items: flex-start;
    gap: 0.7rem;
  }
  .file-name, .file-meta {
    width: 100%;
    white-space: normal;
  }
}
.file-name {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-weight: 600;
}
.file-meta {
  display: flex;
  gap: 1.2rem;
  font-size: 0.95rem;
  color: #b4befe;
}
.file-size, .file-time {
  display: flex;
  align-items: center;
  gap: 0.3rem;
}
.nav-link:hover {
  background: #45475a;
}
@media (max-width: 500px) {
  .container {
    padding: 1rem 0.5rem;
    max-width: 95vw;
  }
  .title {
    font-size: 1.2rem;
  }
  .nav-link {
    font-size: 0.95rem;
    padding: 0.6rem 1rem;
  }
}
</style>
  </head>
  <body>
    <div class="container">
      <h1 class="title">{{ content }}</h1>
      <div class="nav-links">
        {% for file in files %}
          <a class="nav-link" href="{{ file.file }}" download>
      <span class="file-name">
    <i class="fa-solid fa-file"></i>
    {{ file.name }}
  </span>
  <span class="file-meta">
    <span class="file-size">
      <i class="fa-solid fa-database"></i> {{ file.size }}MB
    </span>
    <span class="file-time">
      <i class="fa-solid fa-clock"></i> {{ file.time }}
    </span>
  </span>
</a>
        {% endfor %}
      </div>
    </div>
  </body>
</html>
"#
)]
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
                        .map(|m| m.len() as f64 / 1_000_000 as f64)
                        .unwrap_or(0 as f64);
                    let size_string = format!("{:.2}", size);

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
                        format!("{} minutes ago", minutes_ago)
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
