use rf_common::FileInfo;
use std::collections::HashSet;
use wasm_bindgen_futures::spawn_local;
use web_sys::Element;
use yew::prelude::*;

use crate::components::header::handlers::*;
use crate::shared::models::*;
use crate::shared::tauri_api::*;
use crate::shared::utils::*;

//note コンテキストメニューの消去は <main> の onclick で処理

/// # Summary
///
/// Open のクリックイベントハンドラを生成
pub fn create_open_click_handler(
    all_files: UseStateHandle<Vec<FileInfo>>,
    display_files: UseStateHandle<Vec<FileInfo>>,
    navigation_history: UseStateHandle<NavigationHistory>,
    toasts: UseStateHandle<Vec<Toast>>,
) -> Callback<MouseEvent> {
    //fixme create_file_anchor_double_click_handler() と共通化
    Callback::from(move |e: MouseEvent| {
        let toasts = toasts.clone();
        let push_toast = create_push_toast(toasts);

        // イベントエレメントから必要なデータを取得
        let Some(element) = downcast::<Element>(&e, &push_toast) else {
            return;
        };
        let is_dir = element
            .get_attribute("data-is-dir")
            .map(|v| v == "true")
            .unwrap_or(false);
        let path = element.get_attribute("data-path").unwrap_or_default();

        if !is_dir {
            let path = path.clone();
            spawn_local(async move {
                tc_open_file(path, push_toast.clone()).await;
            });
            return;
        }

        let all_files = all_files.clone();
        let display_files = display_files.clone();
        let mut nh = (*navigation_history).clone();
        nh.push(&path);
        navigation_history.set(nh);
        let path = path.clone();
        spawn_local(async move {
            let file_infos = tc_read_dir(&path, push_toast).await;
            all_files.set(file_infos.clone());
            display_files.set(file_infos);
        });
    })
}

/// # Summary
///
/// Copy のクリックイベントハンドラを生成
pub fn create_copy_click_handler(
    copy_files: UseStateHandle<HashSet<String>>,
    selected_files: UseStateHandle<HashSet<String>>,
) -> Callback<MouseEvent> {
    create_copy_button_click_handler(copy_files, selected_files)
}
