//! rf-ui crate
//!
//! フロントエンド(WASM)から TAURI コマンドを実行するための非同期ラッパーを定義する
use gloo_utils::format::JsValueSerdeExt;
use rf_common::*;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

use crate::shared::models::*;
use crate::system_error;

#[wasm_bindgen]
extern "C" {
    /// エラーが発生せず、引数を取る TAURI コマンドを実行する
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
    /// エラーが発生せず、引数を取らない TAURI コマンドを実行する
    #[wasm_bindgen(js_name = invoke, js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke_no_args(cmd: &str) -> JsValue;
    /// エラーが発生する可能性があり、引数を取る TAURI コマンドを実行する
    #[wasm_bindgen(catch, js_name = invoke, js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke_r(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
    /// エラーが発生する可能性があり、引数を取らない TAURI コマンドを実行する
    #[wasm_bindgen(catch, js_name = invoke, js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke_r_no_args(cmd: &str) -> Result<JsValue, JsValue>;
}

/// 指定したファイルを指定のディレクトリにコピーする
///
/// 引数
///
/// - `files`
///   - コピー元ファイルのフルパス
/// - `to`
///   - コピー先ディレクトリ名称
///   - カレントディレクトリを起点した相対パス
/// - `toaster`
///   - トーストを表示するコールバック関数
///     - エラーメッセージ表示用
///
/// 返却値
///
/// - `bool`
///   - 処理結果
pub async fn tc_copy_files(
    files: Vec<String>,
    to: &String,
    toaster: Callback<(ToastKind, String)>,
) -> bool {
    let args = match JsValue::from_serde(&serde_json::json!({"files": files, "to": to})) {
        Ok(v) => v,
        Err(e) => {
            system_error!(e, &toaster);
            return false;
        }
    };
    invoke_r(TAURI_COMMAND_COPY_FILES, args)
        .await
        .map(|_| true)
        .unwrap_or_else(|e| {
            system_error!(e, &toaster);
            false
        })
}

/// 指定したディレクトリを作成する
///
/// 引数
///
/// - `parent_dir`
///   - 作成するディレクトリの親ディレクトリ
/// - `dir_name`
///   - 作成するディレクトリ名
/// - `toaster`
///   - トーストを表示するコールバック関数
///     - エラーメッセージ表示用
///
/// 返却値
///
/// - `bool`
///   - 処理結果
pub async fn tc_create_dir(
    parent_dir: String,
    dir_name: String,
    toaster: Callback<(ToastKind, String)>,
) -> bool {
    let args = match JsValue::from_serde(
        &serde_json::json!({"parent_dir": parent_dir, "dir_name": dir_name}),
    ) {
        Ok(v) => v,
        Err(e) => {
            system_error!(e, &toaster);
            return false;
        }
    };
    match invoke_r(TAURI_COMMAND_CREATE_DIR, args).await {
        Ok(_) => true,
        Err(e) => {
            system_error!(e, &toaster);
            false
        }
    }
}

/// 指定したファイルを削除する
///
/// 物理削除ではなくゴミ箱へファイルを移動する
///
/// 引数
///
/// - `files`
///   - 対象ファイルのフルパス
/// - `toaster`
///   - トーストを表示するコールバック関数
///     - エラーメッセージ表示用
///
/// 返却値
///
/// - `bool`
///   - 処理結果
pub async fn tc_delete_files(files: Vec<String>, toaster: Callback<(ToastKind, String)>) -> bool {
    let args = match JsValue::from_serde(&serde_json::json!({"files": files})) {
        Ok(v) => v,
        Err(e) => {
            system_error!(e, &toaster);
            return false;
        }
    };
    match invoke_r(TAURI_COMMAND_DELETE_FILES, args).await {
        Ok(_) => true,
        Err(e) => {
            system_error!(e, &toaster);
            false
        }
    }
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
pub async fn tc_get_init_dir() -> String {
    invoke_no_args(TAURI_COMMAND_GET_INIT_DIR)
        .await
        .as_string()
        .unwrap()
}

/// 指定したディレクトリの親ディレクトリを取得する
///
/// 親ディレクトリが存在しない場合、指定したディレクトリを返す
///
/// 引数
///
/// - `dir`
///   - 対象ディレクトリのフルパス
/// - `toaster`
///   - トーストを表示するコールバック関数
///     - エラーメッセージ表示用
///
/// 返却値
///
/// - `String`
///   - 親ディレクトリのフルパス
///   - エラー発生時は空文字
pub async fn tc_get_parent_dir(dir: &str, toaster: Callback<(ToastKind, String)>) -> String {
    let args = match JsValue::from_serde(&serde_json::json!({"dir": dir})) {
        Ok(v) => v,
        Err(e) => {
            system_error!(e, &toaster);
            return String::default();
        }
    };
    invoke(TAURI_COMMAND_GET_PARENT_DIR, args)
        .await
        .as_string()
        .unwrap()
}

//todo return bool
/// 指定したファイルをデフォルトアプリでオープンする
///
/// 引数
///
/// - `file`
///   - 対象ファイルのフルパス
/// - `toaster`
///   - トーストを表示するコールバック関数
///     - エラーメッセージ表示用
pub async fn tc_open_file(file: String, toaster: Callback<(ToastKind, String)>) {
    let args = match JsValue::from_serde(&serde_json::json!({"file": file})) {
        Ok(v) => v,
        Err(e) => {
            system_error!(e, &toaster);
            return;
        }
    };
    let _ = invoke_r(TAURI_COMMAND_OPEN_FILE, args)
        .await
        .inspect_err(|e| {
            system_error!(e, &toaster);
        });
}

/// 指定したディレクトリのファイルリストを取得する
///
/// 引数
///
/// - `dir`
///   - 対象ディレクトリのフルパス
/// - `toaster`
///   - トーストを表示するコールバック関数
///     - エラーメッセージ表示用
///
/// 返却値
///
/// - `Vec<FileInfo>`
///   - ファイルリスト
///   - エラーの場合は空のリスト
pub async fn tc_read_dir(dir: &String, toaster: Callback<(ToastKind, String)>) -> Vec<FileInfo> {
    let args = match JsValue::from_serde(&serde_json::json!({"dir": dir})) {
        Ok(v) => v,
        Err(e) => {
            system_error!(e, &toaster);
            return vec![];
        }
    };
    match invoke_r(TAURI_COMMAND_READ_DIR, args).await {
        Ok(v) => match v.into_serde::<Vec<FileInfo>>() {
            Ok(v) => v,
            Err(e) => {
                system_error!(e, &toaster);
                vec![]
            }
        },
        Err(e) => {
            system_error!(e, &toaster);
            vec![]
        }
    }
}

/// 指定したファイルをリネームする
///
/// 引数
///
/// - `file`
///   - 対象ファイルのフルパス
/// - `new_name`
///   - 変更後のファイル名
/// - `toaster`
///   - トーストを表示するコールバック関数
///     - エラーメッセージ表示用
///
/// 返却値
///
/// - `bool`
///   - 処理結果
pub async fn tc_rename_file(
    file: &String,
    new_name: &String,
    toaster: Callback<(ToastKind, String)>,
) -> bool {
    let args = match JsValue::from_serde(&serde_json::json!({"file": file, "new_name": new_name})) {
        Ok(v) => v,
        Err(e) => {
            system_error!(e, &toaster);
            return false;
        }
    };
    match invoke_r(TAURI_COMMAND_RENAME_FILE, args).await {
        Ok(_) => true,
        Err(e) => {
            system_error!(e, &toaster);
            false
        }
    }
}

/// ファイル選択ダイアログを表示してディレクトリを選択する
///
/// 返却値
///
/// - `Some(String)`
///   - 選択したディレクトリのフルパス
/// - `None`
///   - ディレクトリの選択をキャンセルした場合
pub async fn tc_select_dir() -> Option<String> {
    invoke_no_args(TAURI_COMMAND_SELECT_DIR).await.as_string()
}
