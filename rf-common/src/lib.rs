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
/// FileInfo 構造体のビルダー
#[derive(Default)]
pub struct FileInfoBuilder {
    name: Option<String>,
    path: Option<String>,
    is_dir: Option<bool>,
    is_file: Option<bool>,
    is_symlink: Option<bool>,
    is_block_device: Option<bool>,
    is_char_device: Option<bool>,
    is_fifo: Option<bool>,
    is_socket: Option<bool>,
    size: Option<u64>,
    readonly: Option<bool>,
    mode: Option<u32>,
    accessed: Option<String>,
    created: Option<String>,
    modified: Option<String>,
}

impl FileInfoBuilder {
    pub fn name(mut self, v: impl Into<String>) -> Self {
        self.name = Some(v.into());
        self
    }

    pub fn path(mut self, v: impl Into<String>) -> Self {
        self.path = Some(v.into());
        self
    }

    pub fn is_dir(mut self, v: bool) -> Self {
        self.is_dir = Some(v);
        self
    }

    pub fn is_file(mut self, v: bool) -> Self {
        self.is_file = Some(v);
        self
    }

    pub fn is_symlink(mut self, v: bool) -> Self {
        self.is_symlink = Some(v);
        self
    }

    pub fn is_block_device(mut self, v: bool) -> Self {
        self.is_block_device = Some(v);
        self
    }

    pub fn is_char_device(mut self, v: bool) -> Self {
        self.is_char_device = Some(v);
        self
    }

    pub fn is_fifo(mut self, v: bool) -> Self {
        self.is_fifo = Some(v);
        self
    }

    pub fn is_socket(mut self, v: bool) -> Self {
        self.is_socket = Some(v);
        self
    }

    pub fn size(mut self, v: u64) -> Self {
        self.size = Some(v);
        self
    }

    pub fn readonly(mut self, v: bool) -> Self {
        self.readonly = Some(v);
        self
    }

    pub fn mode(mut self, v: u32) -> Self {
        self.mode = Some(v);
        self
    }

    pub fn accessed(mut self, v: impl Into<String>) -> Self {
        self.accessed = Some(v.into());
        self
    }

    pub fn created(mut self, v: impl Into<String>) -> Self {
        self.created = Some(v.into());
        self
    }

    pub fn modified(mut self, v: impl Into<String>) -> Self {
        self.modified = Some(v.into());
        self
    }

    /// # Summary
    ///
    /// FileInfo インスタンスを生成
    ///
    /// # Returns
    ///
    /// - `FileInfo`: インスタンス
    pub fn build(self) -> FileInfo {
        FileInfo {
            name: self.name.unwrap_or_default(),
            path: self.path.unwrap_or_default(),
            is_dir: self.is_dir.unwrap_or_default(),
            is_file: self.is_file.unwrap_or_default(),
            is_symlink: self.is_symlink.unwrap_or_default(),
            is_block_device: self.is_block_device.unwrap_or_default(),
            is_char_device: self.is_char_device.unwrap_or_default(),
            is_fifo: self.is_fifo.unwrap_or_default(),
            is_socket: self.is_socket.unwrap_or_default(),
            size: self.size.unwrap_or_default(),
            readonly: self.readonly.unwrap_or_default(),
            mode: self.mode.unwrap_or_default(),
            accessed: self.accessed.unwrap_or_default(),
            created: self.created.unwrap_or_default(),
            modified: self.modified.unwrap_or_default(),
        }
    }
}

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
    /// 空のビルダーインスタンスを生成
    ///
    /// # Returns
    ///
    /// - `FileInfoBuilder`: インスタンス
    pub fn builder() -> FileInfoBuilder {
        FileInfoBuilder::default()
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
