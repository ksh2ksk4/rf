use serde::Serialize;
use std::fs;

#[derive(Debug, Serialize)]
struct FileInfo {
    name: String,
    path: String,
    is_dir: bool,
    size: u64,
}

#[tauri::command]
fn read_dir(path: String) -> Result<Vec<FileInfo>, String> {
    let mut entries = Vec::<FileInfo>::new();

    for d in fs::read_dir(&path).map_err(|e| e.to_string())? {
        let de = d.map_err(|e| e.to_string())?;
        let metadata = de.metadata().map_err(|e| e.to_string())?;
        entries.push(FileInfo {
            name: de.file_name().to_string_lossy().to_string(),
            path: de.path().to_string_lossy().to_string(),
            is_dir: metadata.is_dir(),
            size: metadata.len(),
        });
    }

    Ok(entries)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![read_dir])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
