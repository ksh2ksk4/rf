/// # Summary
///
/// デバッグメッセージを表示する
///
/// - 指定した変数の名称と値を開発用ツールのコンソールに出力
/// - debug ビルド用のマクロ
///
/// # Arguments
///
/// - `$var`: 対象の変数
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug {
    ($var:expr) => {
        ::web_sys::console::debug_1(&format!("{}: {:?}", stringify!($var), &$var).into());
    };
}

/// # Summary
///
/// 何もしない
///
/// - release ビルド用のマクロ
///
/// # Arguments
///
/// - `$var`: 対象の変数
#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug {
    ($var:expr) => {
        // no-op(release)
    };
}

/// # Summary
///
/// ワーニングメッセージ(ユーザ用)を表示する
///
/// - ワーニングデータを開発用ツールのコンソールに出力
///
/// # Arguments
///
/// - `$e`: 対象の変数
#[macro_export]
macro_rules! user_warning {
    ($e:expr) => {
        let message = format!("{}", &$e);
        ::web_sys::console::warn_1(&message.clone().into());
    };
}

/// # Summary
///
/// エラーメッセージ(システム用)を表示する
///
/// - エラーデータを開発用ツールのコンソールに出力
/// - エラーデータをトーストでユーザに通知
///
/// # Arguments
///
/// - `$e`: 対象の変数
/// - `$push_toast`: トースト表示用のコールバック関数
#[macro_export]
macro_rules! system_error {
    ($e:expr, $push_toast:expr) => {
        let message = format!("{:?}", &$e);
        ::web_sys::console::error_1(&message.clone().into());
        ($push_toast).emit(($crate::models::ToastKind::Error, message));
    };
}

/// # Summary
///
/// エラーメッセージ(ユーザ用)を表示する
///
/// - エラーデータを開発用ツールのコンソールに出力
/// - エラーデータをトーストでユーザに通知
///
/// # Arguments
///
/// - `$e`: 対象の変数
/// - `$push_toast`: トースト表示用のコールバック関数
#[macro_export]
macro_rules! user_error {
    ($e:expr, $push_toast:expr) => {
        let message = format!("{}", &$e);
        ::web_sys::console::error_1(&message.clone().into());
        ($push_toast).emit(($crate::models::ToastKind::Error, message));
    };
}
