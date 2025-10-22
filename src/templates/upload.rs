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
        <title>crane.rs - upload</title>
        <script src="https://cdn.jsdelivr.net/npm/htmx.org@2.0.7/dist/htmx.min.js"></script>
        <style>
  body {
    background: #181825;
    color: #cdd6f4;
    font-family: "Inter", "Roboto", Arial, sans-serif;
    margin: 0;
    min-height: 100vh;
  }
  .main {
    background: #181825;
    color: #cdd6f4;
    font-family: "Inter", "Roboto", Arial, sans-serif;
    margin: 0;
    min-height: 90vh;
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
  form {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    margin-top: 1.2rem;
  }
  input[type="file"] {
    color: #cdd6f4;
    background: #313244;
    border: 1px solid #45475a;
    border-radius: 6px;
    font-size: 1rem;
    padding: 0.5rem;
  }
  button[type="submit"] {
    background: #313244;
    color: #cdd6f4;
    border: none;
    border-radius: 8px;
    padding: 0.7rem 1.2rem;
    font-size: 1rem;
    font-weight: 500;
    cursor: pointer;
    transition: background 0.2s;
  }
  button[type="submit"]:hover {
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
    button[type="submit"] {
      font-size: 0.95rem;
      padding: 0.6rem 1rem;
    }
  }
  .status {
    border-radius: 8px;
    border: 1px solid #45475a;
    padding: 1rem;
    margin-bottom: 1rem;
    font-weight: 500;
    max-height: 10vh;
    display: none;
  }
</style>
    </head>

    <body>
        <div class="status"></div>
        <div class="main">
            <div>
                <div class="container">
                    <h1 class="title">{{ content }}</h1>
                    <form
                        hx-post="/upload"
                        hx-target=".status"
                        hx-swap="innherHTML"
                        enctype="multipart/form-data"
                    >
                        <input type="file" name="file" required />
                        <button type="submit">Upload</button>
                    </form>
                </div>
            </div>
        </div>
    </body>
    <script>
  const notification = document.querySelector(".status");
  const form = document.querySelector('form[enctype="multipart/form-data"]');
  const fileInput = form.querySelector('input[type="file"]');
  const submitButton = form.querySelector('button[type="submit"]');

  // event listener in form didn't work skill issue I guess or, language issue
  submitButton.addEventListener("click", (e) => {
    if (!fileInput.files.length) {
      e.preventDefault();
      notification.style.display = "block";
      notification.innerHTML = "Please select a file to upload.";
      clearTimeout(window.notificationTimeout);
      window.notificationTimeout = setTimeout(() => {
        notification.innerHTML = "";
        notification.style.display = "none";
      }, 5000);
    } else {
      notification.style.display = "block";
      notification.innerHTML = "Uploading...";
    }
  });

  document.addEventListener("htmx:afterSwap", (e) => {
    if (e.target.classList.contains("status")) {
      clearTimeout(window.notificationTimeout);
      window.notificationTimeout = setTimeout(() => {
        e.target.innerHTML = "";
        e.target.style.display = "none";
      }, 5000);
    }
  });
</script>
</html>
"#
)]
pub struct UploadTemplate {
    content: String,
}
impl UploadTemplate {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}
