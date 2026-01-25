use gloo_utils::format::JsValueSerdeExt;
use rf_common::*;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

use crate::shared::models::*;
use crate::system_error;

#[wasm_bindgen]
extern "C" {
    /**
     * エラーが発生しない TAURI コマンドを実行する場合
     */
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
    #[wasm_bindgen(js_name = invoke, js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke_no_args(cmd: &str) -> JsValue;
    /**
     * エラーが発生する可能性のある TAURI コマンドを実行する場合
     */
    #[wasm_bindgen(catch, js_name = invoke, js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke_r(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(catch, js_name = invoke, js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke_r_no_args(cmd: &str) -> Result<JsValue, JsValue>;
}

/// # Summary
///
/// 指定したファイルを指定したディレクトリにコピーする
///
/// # Arguments
///
/// - `paths`: 対象ファイルのパス
/// - `to`: 対象ディレクトリ
/// - `push_toast`: エラーメッセージ表示用のトースト
///
/// # Returns
///
/// - `bool`: 正常にコピーしたかどうか
pub async fn tc_copy_files(
    paths: Vec<String>,
    to: &String,
    push_toast: Callback<(ToastKind, String)>,
) -> bool {
    let args = match JsValue::from_serde(&serde_json::json!({"paths": paths, "to": to})) {
        Ok(v) => v,
        Err(e) => {
            system_error!(e, &push_toast);
            return false;
        }
    };
    invoke_r(TAURI_COMMAND_COPY_FILES, args)
        .await
        .map(|_| true)
        .unwrap_or_else(|e| {
            system_error!(e, &push_toast);
            false
        })
}

/// # Summary
///
/// 指定したファイルを削除する
///
/// # Arguments
///
/// - `paths`: 対象ファイルのパス
/// - `push_toast`: エラーメッセージ表示用のトースト
///
/// # Returns
///
/// - `bool`: 正常に削除したかどうか
pub async fn tc_delete_files(
    paths: Vec<String>,
    push_toast: Callback<(ToastKind, String)>,
) -> bool {
    let args = match JsValue::from_serde(&serde_json::json!({"paths": paths})) {
        Ok(v) => v,
        Err(e) => {
            system_error!(e, &push_toast);
            return false;
        }
    };
    match invoke_r(TAURI_COMMAND_DELETE_FILES, args).await {
        Ok(_) => true,
        Err(e) => {
            system_error!(e, &push_toast);
            false
        }
    }
}

/// # Summary
///
/// アプリ起動時に表示するディレクトリのパスを返す
///
/// # Returns
///
/// - `String`: パス
pub async fn tc_get_init_path() -> String {
    invoke_no_args(TAURI_COMMAND_GET_INIT_PATH)
        .await
        .as_string()
        .unwrap()
}

/// # Summary
///
/// 指定したディレクトリの親ディレクトリのパスを取得する
///
/// # Arguments
///
/// - `path`: 対象ディレクトリのパス
/// - `push_toast`: エラーメッセージ表示用のトースト
///
/// # Returns
///
/// - `String`: 親ディレクトリのパス(エラーの場合は空文字)
pub async fn tc_get_parent_dir(path: &str, push_toast: Callback<(ToastKind, String)>) -> String {
    let args = match JsValue::from_serde(&serde_json::json!({"path": path})) {
        Ok(v) => v,
        Err(e) => {
            system_error!(e, &push_toast);
            return String::new();
        }
    };
    invoke(TAURI_COMMAND_GET_PARENT_DIR, args)
        .await
        .as_string()
        .unwrap()
}

/// # Summary
///
/// 指定したファイルをデフォルトアプリでオープンする
///
/// # Arguments
///
/// - `path`: 対象ファイルのパス
/// - `push_toast`: エラーメッセージ表示用のトースト
pub async fn tc_open_file(path: String, push_toast: Callback<(ToastKind, String)>) {
    let args = match JsValue::from_serde(&serde_json::json!({"path": path})) {
        Ok(v) => v,
        Err(e) => {
            system_error!(e, &push_toast);
            return;
        }
    };
    let _ = invoke_r(TAURI_COMMAND_OPEN_FILE, args)
        .await
        .inspect_err(|e| {
            system_error!(e, &push_toast);
        });
}

/// # Summary
///
/// 指定したディレクトリのファイルリストを取得する
///
/// # Arguments
///
/// - `path`: 対象ディレクトリのパス
/// - `push_toast`: エラーメッセージ表示用のトースト
///
/// # Returns
///
/// - `Vec<FileInfo>`: ファイルリスト(エラーの場合は空のリスト)
pub async fn tc_read_dir(
    path: &String,
    push_toast: Callback<(ToastKind, String)>,
) -> Vec<FileInfo> {
    let args = match JsValue::from_serde(&serde_json::json!({"path": path})) {
        Ok(v) => v,
        Err(e) => {
            system_error!(e, &push_toast);
            return vec![];
        }
    };
    match invoke_r(TAURI_COMMAND_READ_DIR, args).await {
        Ok(v) => match v.into_serde::<Vec<FileInfo>>() {
            Ok(v) => v,
            Err(e) => {
                system_error!(e, &push_toast);
                vec![]
            }
        },
        Err(e) => {
            system_error!(e, &push_toast);
            vec![]
        }
    }
}

/// # Summary
///
/// 指定したファイルをリネームする
///
/// # Arguments
///
/// - `path`: 対象ファイルのパス
/// - `new_name`: 変更後のファイル名
/// - `push_toast`: エラーメッセージ表示用のトースト
///
/// # Returns
///
/// - `bool`: リネームできたかどうか
pub async fn tc_rename_file(
    path: &String,
    new_name: &String,
    push_toast: Callback<(ToastKind, String)>,
) -> bool {
    let args = match JsValue::from_serde(&serde_json::json!({"path": path, "new_name": new_name})) {
        Ok(v) => v,
        Err(e) => {
            system_error!(e, &push_toast);
            return false;
        }
    };
    match invoke_r(TAURI_COMMAND_RENAME_FILE, args).await {
        Ok(_) => true,
        Err(e) => {
            system_error!(e, &push_toast);
            false
        }
    }
}

/// # Summary
///
/// ファイル選択ダイアログを表示してディレクトリを選択する
///
/// # Returns
///
/// - `String`: 選択したディレクトリのパス
pub async fn tc_select_dir() -> String {
    invoke_no_args(TAURI_COMMAND_SELECT_DIR)
        .await
        .as_string()
        .unwrap()
}
