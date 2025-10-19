use chrono::{DateTime, Local};
use serde::Serialize;
use std::fs;
use std::os::unix::fs::{FileTypeExt, PermissionsExt};

#[derive(Debug, Serialize)]
struct FileInfo {
    name: String,
    path: String,
    is_dir: bool,
    is_file: bool,
    is_symlink: bool,
    // Unix only
    is_block_device: bool,
    // Unix only
    is_char_device: bool,
    // Unix only
    is_fifo: bool,
    // Unix only
    is_socket: bool,
    size: u64,
    readonly: bool,
    // Unix only
    mode: u32,
    accessed: String,
    created: String,
    modified: String,
}

#[tauri::command]
fn read_dir(path: String) -> Result<Vec<FileInfo>, String> {
    let mut entries = Vec::<FileInfo>::new();

    for d in fs::read_dir(&path).map_err(|e| e.to_string())? {
        let de = d.map_err(|e| e.to_string())?;
        let metadata = de.metadata().map_err(|e| e.to_string())?;
        let file_type = metadata.file_type();
        let permissions = metadata.permissions();
        entries.push(FileInfo {
            name: de.file_name().to_string_lossy().to_string(),
            path: de.path().to_string_lossy().to_string(),
            is_dir: metadata.is_dir(),
            is_file: metadata.is_file(),
            is_symlink: metadata.is_symlink(),
            is_block_device: file_type.is_block_device(),
            is_char_device: file_type.is_char_device(),
            is_fifo: file_type.is_fifo(),
            is_socket: file_type.is_socket(),
            size: metadata.len(),
            readonly: permissions.readonly(),
            mode: permissions.mode(),
            // error handling
            accessed: metadata.accessed().map_err(|e| e.to_string()).map(|st| {
                DateTime::<Local>::from(st)
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string()
            })?,
            // error handling
            created: metadata.created().map_err(|e| e.to_string()).map(|st| {
                DateTime::<Local>::from(st)
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string()
            })?,
            // error handling
            modified: metadata.modified().map_err(|e| e.to_string()).map(|st| {
                DateTime::<Local>::from(st)
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string()
            })?,
        });
    }

    Ok(entries)
}

#[tauri::command]
fn select_dir() -> Result<String, String> {
    let path = rfd::FileDialog::new()
        .pick_folder()
        .ok_or("No dir selected")?;
    Ok(path.to_string_lossy().to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![read_dir, select_dir])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
