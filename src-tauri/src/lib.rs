use chrono::{DateTime, Local};
use serde::Serialize;
use std::fs;
use std::os::unix::fs::{FileTypeExt, PermissionsExt};
use std::os::unix::process::ExitStatusExt;
use std::path::Path;
use std::process::Command;
use trash;

/// # Summary
///
/// ファイルに関するデータ
///
/// # Fields
///
/// - `name`: 名前
/// - `path`: パス(フルパス)
/// - `is_dir`: ディレクトリかどうかを表すフラグ
/// - `is_file`: ファイルかどうかを表すフラグ
/// - `is_symlink`: シンボリックリンクかどうかを表すフラグ
/// - `is_block_device`: ブロックデバイスかどうかを表すフラグ(UNIX only)
/// - `is_char_device`: キャラクタデバイスかどうかを表すフラグ(UNIX only)
/// - `is_fifo`: FIFO かどうかを表すフラグ(UNIX only)
/// - `is_socket`: ソケットかどうかを表すフラグ(UNIX only)
/// - `size`: サイズ
/// - `readonly`: 読取専用かどうかを表すフラグ
/// - `mode`: モード(UNIX only)
/// - `accessed`: アクセス日時
/// - `created`: 作成日時
/// - `modified`: 更新日時
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
struct FileInfo {
    name: String,
    path: String,
    is_dir: bool,
    is_file: bool,
    is_symlink: bool,
    is_block_device: bool,
    is_char_device: bool,
    is_fifo: bool,
    is_socket: bool,
    size: u64,
    readonly: bool,
    mode: u32,
    accessed: String,
    created: String,
    modified: String,
}

/// # Summary
///
/// Tauri アプリのエントリポイント
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            delete_files,
            get_parent_dir,
            open_file,
            read_dir,
            select_dir,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// # Summary
///
/// 指定したパスのファイルを削除
/// (物理削除ではなくゴミ箱へ移動)
///
/// # Arguments
///
/// - `paths`: パス(Vec<String>)
///
/// # Returns
///
/// - `Ok(())`: ()
/// - `Err(String)`: エラーメッセージ
#[tauri::command]
fn delete_files(paths: Vec<String>) -> Result<(), String> {
    trash::delete_all(&paths).map_err(|e| e.to_string())?;
    Ok(())
}

/// # Summary
///
/// 親ディレクトリのパスを取得
/// 親ディレクトリが存在しない場合、カレントディレクトリのパスを返す
///
/// # Arguments
///
/// - `path`: パス(String)
///
/// # Returns
///
/// - `String`: 親ディレクトリのパス
#[tauri::command]
fn get_parent_dir(path: String) -> String {
    match Path::new(&path).parent() {
        Some(p) => p.to_string_lossy().to_string(),
        None => path,
    }
}

/// # Summary
///
/// 指定したパスのファイルをオープン
///
/// # Arguments
///
/// - `path`: パス(String)
///
/// # Returns
///
/// - `Ok(())`: ()
/// - `Err(String)`: エラーメッセージ
#[tauri::command]
fn open_file(path: String) -> Result<(), String> {
    open_with_default(&path)
}

#[cfg(target_os = "linux")]
fn open_with_default(path: &str) -> std::io::Result<()> {
    Command::new("xdg-open").arg(path).spawn().map(|_| ())
}

/// # Summary
///
/// 指定したパスのファイルをデフォルトアプリでオープン
///
/// # Arguments
///
/// - `path`: パス(&str)
///
/// # Returns
///
/// - `Ok(())`: ()
/// - `Err(String)`: エラーメッセージ
#[cfg(target_os = "macos")]
fn open_with_default(path: &str) -> Result<(), String> {
    let output = Command::new("open")
        .arg(path)
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        return Ok(());
    }

    let detail = match (output.status.code(), output.status.signal()) {
        // 子プロセスの終了コードが 0 以外の場合
        (Some(c), _) => format!("exit code: {c:?}"),
        // 子プロセスがシグナルで終了した場合
        (None, Some(s)) => format!("terminated by signal {s:?}"),
        // 上記以外
        _ => "failed".to_string(),
    };
    let stderr = String::from_utf8_lossy(&output.stderr);
    Err(format!(
        "open_with_default() - detail: {detail}, stderr: {stderr:?}"
    ))
}

#[cfg(target_os = "windows")]
fn open_with_default(path: &str) -> std::io::Result<()> {
    // start はシェル経由で実行する必要があるので cmd を使う
    Command::new("cmd")
        .args(&["/C", "start", "", path])
        .spawn()
        .map(|_| ())
}

/// # Summary
///
/// 指定したパスのファイルリストを取得
///
/// # Arguments
///
/// - `path`: パス(String)
///
/// # Returns
///
/// - `Ok(Vec<FileInfo>)`: ファイルリスト
/// - `Err(String)`: エラーメッセージ
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
            accessed: metadata.accessed().map_err(|e| e.to_string()).map(|st| {
                DateTime::<Local>::from(st)
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string()
            })?,
            created: metadata.created().map_err(|e| e.to_string()).map(|st| {
                DateTime::<Local>::from(st)
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string()
            })?,
            modified: metadata.modified().map_err(|e| e.to_string()).map(|st| {
                DateTime::<Local>::from(st)
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string()
            })?,
        });
    }

    entries.sort_by(|a, b| a.is_file.cmp(&b.is_file).then_with(|| a.name.cmp(&b.name)));

    Ok(entries)
}

/// # Summary
///
/// ファイルダイアログを表示してパスを選択させる
///
/// # Returns
///
/// - `String`: 選択されたパス
#[tauri::command]
fn select_dir() -> String {
    rfd::FileDialog::new()
        .pick_folder()
        .map_or("".to_string(), |v| v.to_string_lossy().to_string())
}
