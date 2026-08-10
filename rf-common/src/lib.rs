//! rf-common crate
//!
//! フロントエンド(WASM)とバックエンド(TAURI)で共用する構造体や定数を定義する
use serde::{Deserialize, Serialize};

//
// TAURI コマンド
//
pub const TAURI_COMMAND_COPY_FILES: &str = "copy_files";
pub const TAURI_COMMAND_CREATE_DIR: &str = "create_dir";
pub const TAURI_COMMAND_DELETE_FILES: &str = "delete_files";
pub const TAURI_COMMAND_GET_INIT_DIR: &str = "get_init_dir";
pub const TAURI_COMMAND_GET_PARENT_DIR: &str = "get_parent_dir";
pub const TAURI_COMMAND_OPEN_FILE: &str = "open_file";
pub const TAURI_COMMAND_READ_DIR: &str = "read_dir";
pub const TAURI_COMMAND_RENAME_FILE: &str = "rename_file";
pub const TAURI_COMMAND_SELECT_DIR: &str = "select_dir";

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

    /// FileInfo インスタンスを生成
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

/// ファイルに関するデータ
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct FileInfo {
    /// 名称
    name: String,
    /// パス(フルパス)
    path: String,
    /// ディレクトリかどうかを表すフラグ
    is_dir: bool,
    /// ファイルかどうかを表すフラグ
    is_file: bool,
    /// シンボリックリンクかどうかを表すフラグ
    is_symlink: bool,
    /// ブロックデバイスかどうかを表すフラグ(UNIX only)
    is_block_device: bool,
    /// キャラクタデバイスかどうかを表すフラグ(UNIX only)
    is_char_device: bool,
    /// FIFO かどうかを表すフラグ(UNIX only)
    is_fifo: bool,
    /// ソケットかどうかを表すフラグ(UNIX only)
    is_socket: bool,
    /// サイズ
    size: u64,
    /// 読取専用かどうかを表すフラグ
    readonly: bool,
    /// モード(UNIX only)
    mode: u32,
    /// アクセス日時
    accessed: String,
    /// 作成日時
    created: String,
    /// 更新日時
    modified: String,
}

impl FileInfo {
    /// 空のビルダーインスタンスを生成
    pub fn builder() -> FileInfoBuilder {
        FileInfoBuilder::default()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn is_dir(&self) -> bool {
        self.is_dir
    }

    pub fn is_file(&self) -> bool {
        self.is_file
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn accessed(&self) -> String {
        self.accessed.clone()
    }

    pub fn created(&self) -> String {
        self.created.clone()
    }

    pub fn modified(&self) -> String {
        self.modified.clone()
    }
}
