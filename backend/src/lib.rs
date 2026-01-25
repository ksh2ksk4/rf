use chrono::{DateTime, Local};
use rf_common::FileInfo;
use std::ffi::OsString;
use std::fs;
use std::os::unix::fs::{FileTypeExt, PermissionsExt};
use std::os::unix::process::ExitStatusExt;
use std::path::Path;
use std::process::Command;
use trash;

/// # Summary
///
/// Tauri アプリのエントリポイント
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            copy_files,
            delete_files,
            get_init_path,
            get_parent_path,
            open_file,
            read_dir,
            rename_file,
            select_dir,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// # Summary
///
/// 指定したファイルを指定したディレクトリにコピーする
///
/// # Arguments
///
/// - `paths`: 対象ファイルのパス
/// - `to`: 対象ディレクトリ
///
/// # Returns
///
/// - `Ok(())`: `()`
/// - `Err(String)`: エラーメッセージ
#[tauri::command(rename_all = "snake_case")]
fn copy_files(paths: Vec<String>, to: String) -> Result<(), String> {
    let to_path = Path::new(&to);

    if !to_path.exists() {
        return Err(format!("Destination directory does not exist: {to}"));
    }

    if !to_path.is_dir() {
        return Err(format!("Destination does not a directory: {to}"));
    }

    for p in paths {
        let source = Path::new(&p);
        let file_name = source
            .file_name()
            .ok_or_else(|| format!("Source path is invalid: {p}"))?;
        let mut destination = to_path.join(file_name);

        if destination.exists() {
            // コピー先に同名のファイル・ディレクトリが存在する場合、プレフィックスを付与してコピー
            let mut new_file_name = OsString::from("_copied_");
            new_file_name.push(file_name);
            destination = to_path.join(&new_file_name);
        }

        if source.is_dir() {
            copy_dir(source, &destination)?;
        } else if source.is_file() {
            fs::copy(source, &destination).map_err(|e| e.to_string())?;
        } else {
            return Err(format!("Unsupported source file: {p}"));
        }
    }

    Ok(())
}

/// # Summary
///
/// 指定したディレクトリを指定したディレクトリにコピーする
///
/// # Arguments
///
/// - `from`: 対象ディレクトリのパス
/// - `to`: 対象ディレクトリ
///
/// # Returns
///
/// - `Ok(())`: `()`
/// - `Err(String)`: エラーメッセージ
fn copy_dir(from: &Path, to: &Path) -> Result<(), String> {
    fs::create_dir_all(to).map_err(|e| e.to_string())?;

    for v in fs::read_dir(from).map_err(|e| e.to_string())? {
        let entry = v.map_err(|e| e.to_string())?;
        let file_type = entry.file_type().map_err(|e| e.to_string())?;
        let file_name = entry.file_name();
        let mut destination = to.join(&file_name);

        if destination.exists() {
            // コピー先に同名のファイル・ディレクトリが存在する場合、プレフィックスを付与してコピー
            let mut new_file_name = OsString::from("_copied_");
            new_file_name.push(&file_name);
            destination = to.join(&new_file_name);
        }

        let path = entry.path();

        if file_type.is_dir() {
            copy_dir(&path, &destination)?;
        } else {
            fs::copy(&path, &destination).map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}

/// # Summary
///
/// 指定したファイルを削除する
/// (物理削除ではなくゴミ箱へ移動する)
///
/// # Arguments
///
/// - `paths`: 対象ファイルのパス
///
/// # Returns
///
/// - `Ok(())`: `()`
/// - `Err(String)`: エラーメッセージ
#[tauri::command(rename_all = "snake_case")]
fn delete_files(paths: Vec<String>) -> Result<(), String> {
    trash::delete_all(&paths).map_err(|e| e.to_string())?;
    Ok(())
}

/// # Summary
///
/// アプリ起動時に表示するディレクトリのパスを返す
///
/// パスは以下の順で最初に見つかったものを返す
///
/// - 設定ファイルの `general` セクションの `init_path`
///   - 未設定の場合は以下にフォールバック
/// - ユーザのホームディレクトリ
/// - ルートディレクトリ
///
/// # Returns
///
/// - `String`: パス
#[tauri::command(rename_all = "snake_case")]
fn get_init_path() -> String {
    let fallback = dirs::home_dir()
        .map(|v| v.to_string_lossy().into_owned())
        .unwrap_or("/".to_string());

    let contents = match fs::read_to_string("rf.toml") {
        Ok(v) => v,
        Err(_) => return fallback,
    };

    let toml_data = match toml::from_str::<toml::Value>(&contents) {
        Ok(v) => v,
        Err(_) => return fallback,
    };

    if let Some(v) = toml_data
        .get("general")
        .and_then(|v| v.get("init_path"))
        .and_then(|v| v.as_str())
    {
        return v.into();
    }

    fallback
}

/// # Summary
///
/// 指定したディレクトリの親ディレクトリのパスを取得する
/// 親ディレクトリが存在しない場合、指定したディレクトリのパスを返す
///
/// # Arguments
///
/// - `path`: 対象ディレクトリのパス
///
/// # Returns
///
/// - `String`: 親ディレクトリのパス
#[tauri::command(rename_all = "snake_case")]
fn get_parent_path(path: String) -> String {
    match Path::new(&path).parent() {
        Some(p) => p.to_string_lossy().to_string(),
        None => path,
    }
}

/// # Summary
///
/// 指定したファイルをオープンする
///
/// # Arguments
///
/// - `path`: 対象ファイルのパス
///
/// # Returns
///
/// - `Ok(())`: `()`
/// - `Err(String)`: エラーメッセージ
#[tauri::command(rename_all = "snake_case")]
fn open_file(path: String) -> Result<(), String> {
    open_with_default(&path)
}

#[cfg(target_os = "linux")]
fn open_with_default(path: &str) -> std::io::Result<()> {
    Command::new("xdg-open").arg(path).spawn().map(|_| ())
}

/// # Summary
///
/// 指定したファイルをデフォルトアプリでオープンする
///
/// # Arguments
///
/// - `path`: 対象ファイルのパス
///
/// # Returns
///
/// - `Ok(())`: `()`
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
/// 指定したディレクトリのファイルリストを取得する
///
/// # Arguments
///
/// - `path`: 対象ディレクトリのパス
///
/// # Returns
///
/// - `Ok(Vec<FileInfo>)`: ファイルリスト
/// - `Err(String)`: エラーメッセージ
#[tauri::command(rename_all = "snake_case")]
fn read_dir(path: String) -> Result<Vec<FileInfo>, String> {
    let mut entries = Vec::<FileInfo>::new();

    for result in fs::read_dir(&path).map_err(|e| e.to_string())? {
        let dir_entry = result.map_err(|e| e.to_string())?;
        let metadata = dir_entry.metadata().map_err(|e| e.to_string())?;
        let file_type = metadata.file_type();
        let permissions = metadata.permissions();
        entries.push(
            FileInfo::builder()
                .name(dir_entry.file_name().to_string_lossy().to_string())
                .path(dir_entry.path().to_string_lossy().to_string())
                .is_dir(metadata.is_dir())
                .is_file(metadata.is_file())
                .is_symlink(metadata.is_symlink())
                .is_block_device(file_type.is_block_device())
                .is_char_device(file_type.is_char_device())
                .is_fifo(file_type.is_fifo())
                .is_socket(file_type.is_socket())
                .size(metadata.len())
                .readonly(permissions.readonly())
                .mode(permissions.mode())
                .accessed(
                    metadata
                        .accessed()
                        .map(|st| {
                            DateTime::<Local>::from(st)
                                .format("%Y-%m-%d %H:%M:%S")
                                .to_string()
                        })
                        .map_err(|e| e.to_string())?,
                )
                .created(
                    metadata
                        .created()
                        .map(|st| {
                            DateTime::<Local>::from(st)
                                .format("%Y-%m-%d %H:%M:%S")
                                .to_string()
                        })
                        .map_err(|e| e.to_string())?,
                )
                .modified(
                    metadata
                        .modified()
                        .map(|st| {
                            DateTime::<Local>::from(st)
                                .format("%Y-%m-%d %H:%M:%S")
                                .to_string()
                        })
                        .map_err(|e| e.to_string())?,
                )
                .build(),
        );
    }

    entries.sort_by(|a, b| {
        a.is_file()
            .cmp(&b.is_file())
            .then_with(|| a.name().cmp(&b.name()))
    });

    Ok(entries)
}

/// # Summary
///
/// 指定したファイルをリネームする
///
/// # Arguments
///
/// - `path`: 対象ファイルのパス
/// - `new_name`: 変更後のファイル名
///
/// # Returns
///
/// - `Ok(())`: `()`
/// - `Err(String)`: エラーメッセージ
#[tauri::command(rename_all = "snake_case")]
fn rename_file(path: String, new_name: String) -> Result<(), String> {
    let from = Path::new(&path);
    let parent = from
        .parent()
        .ok_or_else(|| "Invalid source path".to_string())?;
    let to = parent.join(new_name);

    if to.exists() {
        return Err("Same name file already exists".to_string());
    }

    fs::rename(from, &to).map_err(|e| e.to_string())?;

    Ok(())
}

/// # Summary
///
/// ファイル選択ダイアログを表示してディレクトリを選択させる
///
/// # Returns
///
/// - `String`: 選択したディレクトリのパス
#[tauri::command(rename_all = "snake_case")]
fn select_dir() -> String {
    rfd::FileDialog::new()
        .pick_folder()
        .map_or("".to_string(), |v| v.to_string_lossy().to_string())
}
