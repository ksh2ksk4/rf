use serde::{Deserialize, Serialize};

// TAURI コマンド
pub const TAURI_COMMAND_COPY_FILES: &str = "copy_files";
pub const TAURI_COMMAND_DELETE_FILES: &str = "delete_files";
pub const TAURI_COMMAND_GET_PARENT_DIR: &str = "get_parent_dir";
pub const TAURI_COMMAND_OPEN_FILE: &str = "open_file";
pub const TAURI_COMMAND_READ_DIR: &str = "read_dir";
pub const TAURI_COMMAND_RENAME_FILE: &str = "rename_file";
pub const TAURI_COMMAND_SELECT_DIR: &str = "select_dir";

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
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct FileInfo {
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

impl FileInfo {
    /// # Summary
    ///
    /// インスタンスを生成
    ///
    /// # Returns
    ///
    /// - `Self`: インスタンス
    pub fn new(
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
    ) -> Self {
        Self {
            name,
            path,
            is_dir,
            is_file,
            is_symlink,
            is_block_device,
            is_char_device,
            is_fifo,
            is_socket,
            size,
            readonly,
            mode,
            accessed,
            created,
            modified,
        }
    }

    /// # Summary
    ///
    ///
    ///
    /// # Returns
    ///
    ///
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// # Summary
    ///
    ///
    ///
    /// # Returns
    ///
    ///
    pub fn path(&self) -> String {
        self.path.clone()
    }

    /// # Summary
    ///
    ///
    ///
    /// # Returns
    ///
    ///
    pub fn is_dir(&self) -> bool {
        self.is_dir
    }

    /// # Summary
    ///
    ///
    ///
    /// # Returns
    ///
    ///
    pub fn is_file(&self) -> bool {
        self.is_file
    }

    /// # Summary
    ///
    ///
    ///
    /// # Returns
    ///
    ///
    pub fn size(&self) -> u64 {
        self.size
    }

    /// # Summary
    ///
    ///
    ///
    /// # Returns
    ///
    ///
    pub fn accessed(&self) -> String {
        self.accessed.clone()
    }

    /// # Summary
    ///
    ///
    ///
    /// # Returns
    ///
    ///
    pub fn created(&self) -> String {
        self.created.clone()
    }

    /// # Summary
    ///
    ///
    ///
    /// # Returns
    ///
    ///
    pub fn modified(&self) -> String {
        self.modified.clone()
    }
}
