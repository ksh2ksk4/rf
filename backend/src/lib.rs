//! rf_lib crate
//!
//! バックエンド(TAURI)の機能(主に TAURI コマンド)を定義する
use chrono::{DateTime, Local};
use rf_common::FileInfo;
use std::ffi::OsString;
use std::fs;
use std::os::unix::fs::{FileTypeExt, PermissionsExt};
use std::os::unix::process::ExitStatusExt;
use std::path::Path;
use std::process::Command;
use trash;

/// Tauri アプリのエントリポイント
///
/// 注意事項
///
/// - tauri::generate_handler!()
///   - Tauri コマンド関数の引数に具象型(具体的にシリアライズ、デシリアライズする方法が明確な型)を期待する
///   - `impl AsRef<Path>` のような抽象型のトレイト境界はマクロで扱えず、マクロ展開時にエラーになる
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            copy_files,
            delete_files,
            get_init_dir,
            get_parent_dir,
            open_file,
            read_dir,
            rename_file,
            select_dir,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// 指定したファイルを指定のディレクトリにコピーする
///
/// 引数
///
/// - `files`
///   - コピー元ファイルのフルパス
/// - `to`
///   - コピー先ディレクトリ名称
///   - カレントディレクトリを起点とした相対パス
///
/// 返却値
///
/// - `Ok(())`
///   - `()`
/// - `Err(String)`
///   - エラーメッセージ
#[tauri::command(rename_all = "snake_case")]
fn copy_files(files: Vec<String>, to: String) -> Result<(), String> {
    /// 指定したディレクトリを指定のディレクトリにコピーする
    ///
    /// TAURI コマンドとして公開しない内部処理用の関数
    ///
    /// 引数
    ///
    /// - `from`
    ///   - コピー元ディレクトリのフルパス
    /// - `to`
    ///   - コピー先ディレクトリのパス
    ///   - カレントディレクトリを起点とした相対パス
    ///
    /// 返却値
    ///
    /// - `Ok(())`
    ///   - ()
    /// - `Err(String)`
    ///   - エラーメッセージ
    fn copy_dir(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<(), String> {
        let from = from.as_ref();
        let to = to.as_ref();

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

    let to_path = Path::new(&to);

    if !to_path.exists() {
        return Err(format!("Destination directory does not exist: {to}"));
    }

    if !to_path.is_dir() {
        return Err(format!("Destination does not a directory: {to}"));
    }

    for p in files {
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

/// 指定したファイルを削除する
///
/// 物理削除ではなくゴミ箱へファイルを移動する
///
/// 引数
///
/// - `files`
///   - 対象ファイルのフルパス
///
/// 返却値
///
/// - `Ok(())`
///   - ()
/// - `Err(String)`
///   - エラーメッセージ
#[tauri::command(rename_all = "snake_case")]
fn delete_files(files: Vec<String>) -> Result<(), String> {
    trash::delete_all(&files).map_err(|e| e.to_string())?;
    Ok(())
}

/// アプリ起動時に表示するディレクトリを取得する
///
/// 以下の順で最初に見つかったディレクトリを返す
///
/// - 設定ファイルの `general` セクションの `init_dir`
///   - 未設定の場合は以下にフォールバック
/// - ユーザのホームディレクトリ
/// - ルートディレクトリ
///
/// 返却値
///
/// - `String`
///   - ディレクトリのフルパス
#[tauri::command(rename_all = "snake_case")]
fn get_init_dir() -> String {
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

/// 指定したディレクトリの親ディレクトリを取得する
///
/// 親ディレクトリが存在しない場合、指定したディレクトリを返す
///
/// 引数
///
/// - `dir`
///   - 対象ディレクトリのフルパス
///
/// 返却値
///
/// - `String`
///   - 親ディレクトリのフルパス
#[tauri::command(rename_all = "snake_case")]
fn get_parent_dir(dir: String) -> String {
    match Path::new(&dir).parent() {
        Some(p) => p.to_string_lossy().into_owned(),
        None => dir,
    }
}

/// 指定したファイルをデフォルトアプリでオープンする
///
/// open_with_default_app() のラッパー
///
/// 引数
///
/// - `file`
///   - 対象ファイルのフルパス
///
/// 返却値
///
/// - `Ok(())`
///   - `()`
/// - `Err(String)`
///   - エラーメッセージ
#[tauri::command(rename_all = "snake_case")]
fn open_file(file: String) -> Result<(), String> {
    /// 指定したファイルをデフォルトアプリでオープンする
    ///
    /// 引数
    ///
    /// - `file`
    ///   - 対象ファイルのフルパス
    ///
    /// 返却値
    ///
    /// - `Ok(())`
    ///   - `()`
    /// - `Err(String)`
    ///   - エラーメッセージ
    #[cfg(target_os = "macos")]
    fn open_with_default_app(file: &str) -> Result<(), String> {
        let output = Command::new("open")
            .arg(file)
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
            "open_with_default_app() - detail: {detail}, stderr: {stderr:?}"
        ))
    }

    #[cfg(target_os = "windows")]
    fn open_with_default_app(path: &str) -> std::io::Result<()> {
        // start はシェル経由で実行する必要があるので cmd を使う
        Command::new("cmd")
            .args(&["/C", "start", "", path])
            .spawn()
            .map(|_| ())
    }

    #[cfg(target_os = "linux")]
    fn open_with_default_app(path: &str) -> std::io::Result<()> {
        Command::new("xdg-open").arg(path).spawn().map(|_| ())
    }

    open_with_default_app(&file)
}

/// 指定したディレクトリのファイルリストを取得する
///
/// 引数
///
/// - `dir`
///   - 対象ディレクトリのフルパス
///
/// 返却値
///
/// - `Ok(Vec<FileInfo>)`
///   - ファイルリスト
/// - `Err(String)`
///   - エラーメッセージ
#[tauri::command(rename_all = "snake_case")]
fn read_dir(dir: String) -> Result<Vec<FileInfo>, String> {
    let mut entries = Vec::<FileInfo>::new();

    for result in fs::read_dir(&dir).map_err(|e| e.to_string())? {
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

/// 指定したファイルをリネームする
///
/// 引数
///
/// - `file`
///   - 対象ファイルのフルパス
/// - `new_name`
///   - 変更後のファイル名
///
/// 返却値
///
/// - `Ok(())`
///   - `()`
/// - `Err(String)`
///   - エラーメッセージ
#[tauri::command(rename_all = "snake_case")]
fn rename_file(file: String, new_name: String) -> Result<(), String> {
    let from = Path::new(&file);
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

/// ファイル選択ダイアログを表示してディレクトリを選択させる
///
/// 返却値
///
/// - `String`
///   - 選択したディレクトリのフルパス
#[tauri::command(rename_all = "snake_case")]
fn select_dir() -> String {
    rfd::FileDialog::new()
        .pick_folder()
        .map_or("".to_string(), |v| v.to_string_lossy().to_string())
}

/// ユニットテスト
#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use std::ffi::OsString;
    use std::path::PathBuf;
    use std::time::SystemTime;
    use std::{env, fs};

    const CONFIG_FILE: &str = "rf.toml";
    const ENV_HOME: &str = "HOME";

    struct TestEnvGuard {
        // テスト開始前のカレントワーキングディレクトリ
        cwd: PathBuf,
        // テスト開始前のホームディレクトリ
        home_dir: Option<OsString>,
        // テスト用のカレントワーキングディレクトリ
        test_cwd: PathBuf,
        // テスト用のホームディレクトリ
        test_home_dir: String,
        // テスト用の初期設定ディレクトリ
        test_init_dir: String,
    }

    impl TestEnvGuard {
        /// テスト用のカレントワーキングディレクトリを準備し、そのフルパスを返す
        fn setup_test_cwd() -> PathBuf {
            let test_cwd = env::temp_dir().join(format!(
                "rf_lib_{}",
                SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ));
            dbg!(&test_cwd);

            let _ = fs::create_dir_all(&test_cwd);
            let _ = env::set_current_dir(&test_cwd);

            test_cwd
        }

        /// テスト用のホームディレクトリを準備し、そのフルパスを返す
        fn setup_test_home_dir() -> String {
            let test_home_dir = env::temp_dir()
                .join("rf_lib_test_home")
                .to_string_lossy()
                .into_owned();
            dbg!(&test_home_dir);

            env::set_var(ENV_HOME, &test_home_dir);

            test_home_dir
        }

        /// テスト用の初期設定ディレクトリのフルパスを返す
        fn setup_test_init_dir() -> String {
            let test_init_dir = env::temp_dir()
                .join("rf_lib_test_init")
                .to_string_lossy()
                .into_owned();
            dbg!(&test_init_dir);

            test_init_dir
        }

        /// テスト環境を準備する
        ///
        /// - テスト実行前の設定を退避
        /// - テスト用のカレントワーキングディレクトリに移動
        /// - 環境変数 `HOME` にテスト用のホームディレクトリを設定
        fn new() -> Self {
            let cwd = env::current_dir().unwrap();
            dbg!(&cwd);
            let home_dir = env::var_os(ENV_HOME);
            dbg!(&home_dir);

            Self {
                cwd,
                home_dir,
                test_cwd: Self::setup_test_cwd(),
                test_home_dir: Self::setup_test_home_dir(),
                test_init_dir: Self::setup_test_init_dir(),
            }
        }

        /// テスト用のホームディレクトリのフルパスを返す
        ///
        /// e.g. /var/folders/yh/jldgtx9d1rq8yyc22jyk_kb40000gn/T/rf_lib_test_home
        fn get_test_home_dir(&self) -> &String {
            &self.test_home_dir
        }

        /// テスト用の初期設定ディレクトリのフルパスを返す
        ///
        /// e.g. /var/folders/yh/jldgtx9d1rq8yyc22jyk_kb40000gn/T/rf_lib_test_init
        fn get_test_init_dir(&self) -> &String {
            &self.test_init_dir
        }

        /// テスト用の設定ファイルを作成する
        ///
        /// 初期設定ディレクトリの設定あり
        fn write_test_config_file(&self) {
            let test_config_file = self.test_cwd.join(CONFIG_FILE);
            let contents = format!(
                indoc! { r##"
                    [general]
                    init_path = "{}"
                "##},
                self.test_init_dir
            );
            let _ = fs::write(&test_config_file, contents);
        }

        /// テスト用の設定ファイルを作成する
        ///
        /// 初期設定ディレクトリの設定なし
        fn write_test_config_file_without_init_dir(&self) {
            let test_config_file = self.test_cwd.join(CONFIG_FILE);
            let contents = format!(indoc! { r##"
                    [general]
                "##});
            let _ = fs::write(&test_config_file, contents);
        }
    }

    impl Drop for TestEnvGuard {
        /// テスト環境を破棄する
        fn drop(&mut self) {
            if let Some(v) = &self.home_dir {
                env::set_var(ENV_HOME, v);
            } else {
                env::remove_var(ENV_HOME);
            }
            let _ = env::set_current_dir(&self.cwd);
            let _ = fs::remove_dir_all(&self.test_cwd);
        }
    }

    /// delete_files() のユニットテスト
    ///
    /// trash::delete_all() を実行しているのみのためテスト対象外とする
    mod delete_files_tests {}

    /// get_init_dir() のユニットテスト
    ///
    /// ルートディレクトリへのフォールバックはテスト対象外とする
    mod get_init_dir_tests {
        use super::*;

        /// - 設定ファイルあり
        ///   - 設定あり
        ///     - その設定値を返す
        #[test]
        fn test_get_init_dir_case_01() {
            let guard = TestEnvGuard::new();
            guard.write_test_config_file();

            let init_dir = get_init_dir();
            dbg!(&init_dir);
            assert_eq!(init_dir, *guard.get_test_init_dir());
        }

        // - 設定ファイルあり
        //   - 設定なし
        //     - ユーザのホームディレクトリを返す
        #[test]
        fn test_get_init_dir_case_02() {
            let guard = TestEnvGuard::new();
            guard.write_test_config_file_without_init_dir();

            let init_dir = get_init_dir();
            dbg!(&init_dir);
            assert_eq!(init_dir, *guard.get_test_home_dir());
        }

        /// - 設定ファイルなし
        ///   - ユーザのホームディレクトリを返す
        #[test]
        fn test_get_init_dir_case_03() {
            let guard = TestEnvGuard::new();

            let init_dir = get_init_dir();
            dbg!(&init_dir);
            assert_eq!(init_dir, *guard.get_test_home_dir());
        }
    }

    /// get_parent_dir() のユニットテスト
    mod get_parent_dir_tests {
        use std::path::MAIN_SEPARATOR;

        use super::*;

        /// - 親ディレクトリあり
        ///   - 親ディレクトリを返す
        #[test]
        fn test_get_parent_dir_case_01() {
            let guard = TestEnvGuard::new();

            let parent_dir = get_parent_dir(guard.get_test_home_dir().clone());
            dbg!(&parent_dir);
            assert_eq!(
                parent_dir,
                env::temp_dir()
                    .to_string_lossy()
                    .trim_end_matches(MAIN_SEPARATOR)
                    .to_owned()
            );
        }

        /// - 親ディレクトリなし
        ///   - 指定したディレクトリを返す
        #[test]
        fn test_get_parent_dir_case_02() {
            let parent_dir = get_parent_dir("/".into());
            dbg!(&parent_dir);
            assert_eq!(parent_dir, "/");
        }
    }

    /// open_file() のユニットテスト
    ///
    /// ユニットテストの実装が難しいためパス
    mod open_file_tests {
        /// open_with_default_app() のユニットテスト
        ///
        /// ユニットテストの実装が難しいためパス
        mod open_with_default_app_tests {}
    }
}
