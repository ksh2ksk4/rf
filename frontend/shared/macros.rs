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
/// ワーニングメッセージを開発用ツールのコンソールに出力する
///
/// # Arguments
///
/// - `$message`: ワーニングメッセージ
///   - `String` に変換可能なデータ
#[macro_export]
macro_rules! warning {
    ($message:expr) => {
        let message: String = ($message).into();
        ::web_sys::console::warn_1(&message.into());
    };
}

/// # Summary
///
/// システムエラーを表示する
///
/// - システムエラーを開発用ツールのコンソールに出力
/// - システムエラーをトーストでユーザに通知
///
/// # Arguments
///
/// - `$e`: エラーデータまたはエラーメッセージ
/// - `$push_toast`: トースト表示用のコールバック関数
#[macro_export]
macro_rules! system_error {
    ($e:expr, $push_toast:expr) => {
        let message = format!("{:?}", &$e);
        ::web_sys::console::error_1(&message.clone().into());
        (&$push_toast).emit(($crate::shared::models::ToastKind::Error, message));
    };
}

/// # Summary
///
/// ユーザに対してエラーメッセージを表示する
///
/// - エラーメッセージを開発用ツールのコンソールに出力
/// - エラーメッセージをトーストでユーザに通知
///
/// # Arguments
///
/// - `$message`: エラーメッセージ
///   - `String` に変換可能なデータ
/// - `$push_toast`: トースト表示用のコールバック関数
#[macro_export]
macro_rules! user_error {
    ($message:expr, $push_toast:expr) => {
        let message: String = ($message).into();
        ::web_sys::console::error_1(&message.clone().into());
        (&$push_toast).emit(($crate::shared::models::ToastKind::Error, message));
    };
}
