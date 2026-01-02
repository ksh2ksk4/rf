use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicUsize, Ordering};

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

    /// # Summary
    ///
    /// インスタンスを生成
    ///
    /// # Returns
    ///
    /// - `Self`: インスタンス
    fn new() -> Self {
        Self {
            index: Default::default(),
            paths: vec![Self::INIT_PATH.to_string()],
        }
    }
}

// Default トレイトを実装
impl Default for NavigationHistory {
    fn default() -> Self {
        Self::new()
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
