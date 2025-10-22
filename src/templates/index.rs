use askama::Template;
#[derive(Template)]
#[template(
    ext = "html",
    source = r#"

<!doctype html>
<html lang="en">
    <head>
        <meta charset="UTF-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <title>crane.rs - index</title>
        <style>
  body {
    background: #181825;
    color: #cdd6f4;
    font-family: "Inter", "Roboto", Arial, sans-serif;
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
    background: #313244;
    color: #cdd6f4;
    border: none;
    border-radius: 8px;
    padding: 0.7rem 1.2rem;
    font-size: 1rem;
    text-decoration: none;
    font-weight: 500;
    transition: background 0.2s;
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
                <a class="nav-link" href="/upload">Go to Upload Page</a>
                <a class="nav-link" href="/download">Go to Download Page</a>
            </div>
        </div>
    </body>
</html>
"#
)]
pub struct IndexTemplate {
    content: String,
}
impl IndexTemplate {
    pub fn new(content: String) -> Self {
        IndexTemplate { content }
    }
}
