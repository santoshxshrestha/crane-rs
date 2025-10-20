use crate::PathBuf;
use crate::env;
use crate::fs;

pub fn copy_files_to_temp(files: Vec<PathBuf>) -> std::io::Result<()> {
    let tmp_dir = env::temp_dir().join("crane-rs");

    if let Err(e) = fs::create_dir_all(&tmp_dir) {
        eprintln!("Failed to create directory: {}", e);
        return Err(e);
    };

    for file in files {
        let file_name = file
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let dest_path = tmp_dir.join(&file_name);
        fs::copy(&file, &dest_path)?;
    }
    Ok(())
}
