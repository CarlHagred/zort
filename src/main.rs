use std::env::args;
use std::fs::{create_dir_all, read_dir, rename};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let argument = match args().nth(1) {
        Some(arg) => arg,
        None => {
            println!("A path needs to be provided as the first argument");
            return Ok(());
        }
    };
    let path_content = read_dir(&argument)?;
    let base_dir = std::path::Path::new(&argument);
    for content in path_content {
        let entry = content?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if let Some(ext_str) = path.extension().and_then(|ext| ext.to_str()) {
            let target_folder = match ext_str {
                "jpg" | "png" | "gif" => "Images",
                "pdf" | "doc" | "txt" | "docx" => "Documents",
                "csv" | "xlsx" | "xlx" | "ods" => "Sheets",
                "mp3" | "wav" | "flac" | "aac" => "Music",
                "mp4" | "mov" | "avi" | "mkv" | "wmv" => "Video",
                "zip" | "rar" | "7z" | "gz" => "Zipped",
                "exe" | "app" | "bat" | "dll" => "Executable",
                _ => "Other",
            };
            let target_dir_path = base_dir.join(target_folder);
            create_dir_all(&target_dir_path)?;
            let file_name = match path.file_name() {
                Some(file_name) => file_name,
                None => {
                    println!("Could not extract filename from {}", path.display());
                    return Ok(());
                }
            };
            let destination_path = target_dir_path.join(file_name);
            rename(&path, destination_path)?;
            println!("Moved {} to {}", file_name.to_string_lossy(), target_folder);
        }
    }
    Ok(())
}
