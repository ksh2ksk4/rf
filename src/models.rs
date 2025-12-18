use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicUsize, Ordering};

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
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
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

/// # Summary
///
/// 表示履歴に関するデータ
///
/// # Fields
///
/// - `index`: 表示中のパスを指し示すインデックス
/// - `paths`: 表示履歴のパスのリスト
#[derive(Clone, Debug, PartialEq)]
pub struct NavigationHistory {
    index: usize,
    paths: Vec<String>,
}

impl NavigationHistory {
    // 初期表示パス
    const INIT_PATH: &str = "/Users/ksh2ksk4/Downloads";

    /// # Summary
    ///
    /// インスタンスを生成
    ///
    /// # Returns
    ///
    /// - `Self`: インスタンス
    pub fn new() -> Self {
        Self {
            index: Default::default(),
            paths: vec![Self::INIT_PATH.to_string()],
        }
    }

    /// # Summary
    ///
    /// 表示履歴のパスのリストを返す
    ///
    /// - 内部の `Vec<String>` をクローンして返すため、コピーコストがある
    /// - 履歴は最古から最新の順序になっている
    ///
    /// # Returns
    ///
    /// - `Vec<String>`: 表示履歴のパスのリスト
    pub fn paths(&self) -> Vec<String> {
        self.paths.clone()
    }

    /// # Summary
    ///
    /// 一つ前のパスに戻れるかチェック
    ///
    /// # Returns
    ///
    /// - `bool`: 一つ前のパスに戻れるかどうか
    pub fn can_back(&self) -> bool {
        self.index > 0
    }

    /// # Summary
    ///
    /// 一つ後のパスに進めるかチェック
    ///
    /// # Returns
    ///
    /// - `bool`: 一つ後のパスに進めるかどうか
    pub fn can_forward(&self) -> bool {
        self.index + 1 < self.paths.len()
    }

    /// # Summary
    ///
    /// 現在のパスを返す
    ///
    /// # Returns
    ///
    /// - `&str`: パス
    pub fn current(&self) -> &str {
        &self.paths[self.index]
    }

    /// # Summary
    ///
    /// 一つ前のパスに戻る
    ///
    /// # Returns
    ///
    /// - `String`: 一つ前のパス。前のパスがない場合は現在のパスを返す
    pub fn back(&mut self) -> String {
        if self.can_back() {
            self.index -= 1;
        }

        self.current().to_string()
    }

    /// # Summary
    ///
    /// 一つ後のパスに進む
    ///
    /// # Returns
    ///
    /// - `String`: 一つ後のパス。後のパスがない場合は現在のパスを返す
    pub fn forward(&mut self) -> String {
        if self.can_forward() {
            self.index += 1;
        }

        self.current().to_string()
    }

    /// # Summary
    ///
    /// 履歴にパスを追加
    ///
    /// # Arguments
    ///
    /// - `path`: パス
    pub fn push(&mut self, path: &str) {
        if self.index + 1 < self.paths.len() {
            // 最新の移動履歴ではない場合
            self.paths.truncate(self.index + 1);
        }

        self.paths.push(path.to_string());
        self.index = self.paths.len() - 1;
    }
}

// トースト ID のアトミックカウンタ
static TOAST_ID: Lazy<AtomicUsize> = Lazy::new(|| Default::default());

/// # Summary
///
/// トーストの種類
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ToastKind {
    Success,
    Info,
    Warning,
    Error,
}

/// # Summary
///
/// 個々のトーストに関するデータ
///
/// # Fields
///
/// - `id`: ID
/// - `kind`: 種類
/// - `message`: メッセージ
#[derive(Clone, Debug, PartialEq)]
pub struct Toast {
    id: usize,
    kind: ToastKind,
    message: String,
}

impl Toast {
    // トーストを表示する時間(ms)
    pub const DURATION: u32 = 5000;

    /// # Summary
    ///
    ///
    ///
    /// # Returns
    ///
    ///
    pub fn new(kind: ToastKind, message: impl Into<String>) -> Self {
        Toast {
            id: TOAST_ID.fetch_add(1, Ordering::Relaxed),
            kind,
            message: message.into(),
        }
    }

    /// # Summary
    ///
    ///
    ///
    /// # Returns
    ///
    ///
    pub fn id(&self) -> usize {
        self.id
    }

    /// # Summary
    ///
    ///
    ///
    /// # Returns
    ///
    ///
    pub fn kind(&self) -> ToastKind {
        self.kind
    }

    /// # Summary
    ///
    ///
    ///
    /// # Returns
    ///
    ///
    pub fn message(&self) -> String {
        self.message.clone()
    }

    /// # Summary
    ///
    ///
    ///
    /// # Returns
    ///
    ///
    pub fn next_id() -> usize {
        TOAST_ID.load(Ordering::Relaxed)
    }
}
