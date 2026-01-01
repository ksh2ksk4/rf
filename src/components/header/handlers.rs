use rf_common::FileInfo;
use std::collections::HashSet;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, InputEvent};
use yew::prelude::*;

use crate::shared::models::*;
use crate::shared::tauri_api::*;
use crate::shared::utils::*;

/// # Summary
///
/// back ボタンのクリックイベントハンドラを生成
pub fn create_back_button_click_handler(
    all_files: UseStateHandle<Vec<FileInfo>>,
    display_files: UseStateHandle<Vec<FileInfo>>,
    navigation_history: UseStateHandle<NavigationHistory>,
    toasts: UseStateHandle<Vec<Toast>>,
) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        let mut nh = (*navigation_history).clone();
        let path = nh.back();
        navigation_history.set(nh);

        let all_files = all_files.clone();
        let display_files = display_files.clone();
        let toasts = toasts.clone();
        spawn_local(async move {
            update_file_list(&path, all_files, display_files, toasts).await;
        });
    })
}

/// # Summary
///
/// forward ボタンのクリックイベントハンドラを生成
pub fn create_forward_button_click_handler(
    all_files: UseStateHandle<Vec<FileInfo>>,
    display_files: UseStateHandle<Vec<FileInfo>>,
    navigation_history: UseStateHandle<NavigationHistory>,
    toasts: UseStateHandle<Vec<Toast>>,
) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        let mut nh = (*navigation_history).clone();
        let path = nh.forward();
        navigation_history.set(nh);

        let all_files = all_files.clone();
        let display_files = display_files.clone();
        let toasts = toasts.clone();
        spawn_local(async move {
            update_file_list(&path, all_files, display_files, toasts).await;
        });
    })
}

/// # Summary
///
/// "go to parent dir" ボタンのクリックイベントハンドラを生成
pub fn create_go_to_parent_dir_button_click_handler(
    all_files: UseStateHandle<Vec<FileInfo>>,
    display_files: UseStateHandle<Vec<FileInfo>>,
    navigation_history: UseStateHandle<NavigationHistory>,
    toasts: UseStateHandle<Vec<Toast>>,
) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        let all_files = all_files.clone();
        let display_files = display_files.clone();
        let navigation_history = navigation_history.clone();
        let toasts = toasts.clone();
        spawn_local(async move {
            let path = tc_get_parent_dir(
                navigation_history.current(),
                create_push_toast(toasts.clone()),
            )
            .await;
            update_file_list(&path, all_files, display_files, toasts).await;

            let mut nh = (*navigation_history).clone();
            nh.push(&path);
            navigation_history.set(nh);
        });
    })
}

/// # Summary
///
/// "select dir" ボタンのクリックイベントハンドラを生成
pub fn create_select_dir_button_click_handler(
    all_files: UseStateHandle<Vec<FileInfo>>,
    display_files: UseStateHandle<Vec<FileInfo>>,
    navigation_history: UseStateHandle<NavigationHistory>,
    toasts: UseStateHandle<Vec<Toast>>,
) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        let all_files = all_files.clone();
        let display_files = display_files.clone();
        let navigation_history = navigation_history.clone();
        let toasts = toasts.clone();
        spawn_local(async move {
            let path = tc_select_dir().await;
            update_file_list(&path, all_files, display_files, toasts).await;

            let mut nh = (*navigation_history).clone();
            nh.push(&path);
            navigation_history.set(nh);
        });
    })
}

/// # Summary
///
/// reload ボタンのクリックイベントハンドラを生成
pub fn create_reload_button_click_handler(
    all_files: UseStateHandle<Vec<FileInfo>>,
    display_files: UseStateHandle<Vec<FileInfo>>,
    navigation_history: UseStateHandle<NavigationHistory>,
    toasts: UseStateHandle<Vec<Toast>>,
) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        let all_files = all_files.clone();
        let display_files = display_files.clone();
        let nh = (*navigation_history).clone();
        let path = nh.current().to_string();
        let toasts = toasts.clone();
        spawn_local(async move {
            update_file_list(&path, all_files, display_files, toasts).await;
        });
    })
}

/// # Summary
///
/// copy ボタンのクリックイベントハンドラを生成
pub fn create_copy_button_click_handler(
    copy_files: UseStateHandle<HashSet<String>>,
    selected_files: UseStateHandle<HashSet<String>>,
) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        copy_files.set((*selected_files).clone());
    })
}

/// # Summary
///
/// paste ボタンのクリックイベントハンドラを生成
pub fn create_paste_button_click_handler(
    all_files: UseStateHandle<Vec<FileInfo>>,
    copy_files: UseStateHandle<HashSet<String>>,
    display_files: UseStateHandle<Vec<FileInfo>>,
    navigation_history: UseStateHandle<NavigationHistory>,
    selected_files: UseStateHandle<HashSet<String>>,
    toasts: UseStateHandle<Vec<Toast>>,
) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        let nh = (*navigation_history).clone();
        let current_path = nh.current().to_string();

        let copy_files = copy_files.clone();
        let paths: Vec<String> = (*copy_files).iter().cloned().collect();

        let all_files = all_files.clone();
        let display_files = display_files.clone();
        let selected_files = selected_files.clone();
        let toasts = toasts.clone();
        spawn_local(async move {
            tc_copy_files(paths, &current_path, create_push_toast(toasts.clone())).await;
            update_file_list(&current_path, all_files, display_files, toasts).await;
            // 選択状態をクリア
            copy_files.set(Default::default());
            selected_files.set(Default::default());
        });
    })
}

/// # Summary
///
/// "delete files" ボタンのクリックイベントハンドラを生成
pub fn create_delete_files_button_click_handler(
    all_files: UseStateHandle<Vec<FileInfo>>,
    display_files: UseStateHandle<Vec<FileInfo>>,
    navigation_history: UseStateHandle<NavigationHistory>,
    selected_files: UseStateHandle<HashSet<String>>,
    toasts: UseStateHandle<Vec<Toast>>,
) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        let nh = (*navigation_history).clone();
        let current_path = nh.current().to_string();

        let selected_files = selected_files.clone();
        let paths: Vec<String> = (*selected_files).iter().cloned().collect();

        let all_files = all_files.clone();
        let display_files = display_files.clone();
        let toasts = toasts.clone();
        spawn_local(async move {
            if tc_delete_files(paths, create_push_toast(toasts.clone())).await {
                update_file_list(&current_path, all_files, display_files, toasts).await;
                // 選択状態をクリア
                selected_files.set(HashSet::new());
            }
        });
    })
}

/// # Summary
///
/// フィルタテキストボックスの入力イベントハンドラを生成
pub fn create_filter_textbox_input_handler(
    filter: UseStateHandle<String>,
    toasts: UseStateHandle<Vec<Toast>>,
) -> Callback<InputEvent> {
    Callback::from(move |e: InputEvent| {
        downcast::<HtmlInputElement>(&e, &create_push_toast(toasts.clone())).inspect(|v| {
            filter.set(v.value());
        });
    })
}

/// # Summary
///
/// 指定したディレクトリに存在するファイルでファイル一覧関連データを更新する
///
/// # Arguments
///
/// - `path`: 対象ディレクトリのパス
/// - `all_files`: 対象ディレクトリのすべてのファイルを保持するステート
/// - `display_files`: ファイルリストに表示するファイルを保持するステート
/// - `toasts`: 表示待ちのトーストを保持するステート
async fn update_file_list(
    path: &String,
    all_files: UseStateHandle<Vec<FileInfo>>,
    display_files: UseStateHandle<Vec<FileInfo>>,
    toasts: UseStateHandle<Vec<Toast>>,
) {
    let file_infos = tc_read_dir(path, create_push_toast(toasts)).await;
    all_files.set(file_infos.clone());
    display_files.set(file_infos);
}
