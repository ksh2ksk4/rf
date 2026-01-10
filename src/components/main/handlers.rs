use gloo_timers::callback::Timeout;
use rf_common::FileInfo;
use std::cell::RefCell;
use std::collections::HashSet;
use std::path::Path;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Element, Event, HtmlInputElement};
use yew::prelude::*;

use crate::components::context_menu::fc::*;
use crate::shared::models::*;
use crate::shared::tauri_api::*;
use crate::shared::utils::*;
use crate::{user_error, warning};

/// # Summary
///
/// ファイルチェックボックスのクリックイベントハンドラを生成
pub fn create_file_checkbox_click_handler(
    selected_files: UseStateHandle<HashSet<String>>,
    toasts: UseStateHandle<Vec<Toast>>,
) -> Callback<Event> {
    Callback::from(move |e: Event| {
        // イベントエレメントから必要なデータを取得
        let Some(element) = downcast::<HtmlInputElement>(&e, &create_push_toast(toasts.clone()))
        else {
            return;
        };
        let checked = element.checked();
        let path = element.get_attribute("data-path").unwrap_or_default();

        let mut new_value = (*selected_files).clone();

        if checked {
            new_value.insert(path);
        } else {
            new_value.remove(&path);
        }

        selected_files.set(new_value);
    })
}

/// # Summary
///
/// ファイルのクリックイベントハンドラを生成
pub fn create_file_anchor_click_handler(
    renaming_file: UseStateHandle<Option<String>>,
    selected_files: UseStateHandle<HashSet<String>>,
    toasts: UseStateHandle<Vec<Toast>>,
    click_timeout: Rc<RefCell<Option<Timeout>>>,
) -> Callback<MouseEvent> {
    Callback::from(move |e: MouseEvent| {
        e.prevent_default();

        // イベントエレメントから必要なデータを取得
        let Some(element) = downcast::<Element>(&e, &create_push_toast(toasts.clone())) else {
            return;
        };
        let path = element.get_attribute("data-path").unwrap_or_default();

        if let Some(v) = click_timeout.borrow_mut().take() {
            // 既にタイマーがある場合
            v.cancel();
        }

        let renaming_file = renaming_file.clone();
        let selected_files = selected_files.clone();
        // シングルクリックの処理を 250ms 保留
        *click_timeout.borrow_mut() = Some(Timeout::new(250, move || {
            let mut new_value = HashSet::<String>::new();
            new_value.insert(path.clone());

            if (*selected_files).clone() == new_value {
                // 選択しているファイルを再度クリックした場合
                renaming_file.set(Some(path));
            }

            selected_files.set(new_value);
        }));
    })
}

/// # Summary
///
/// ファイルの右クリックイベントハンドラを生成
pub fn create_file_anchor_context_menu_handler(
    copy_files: UseStateHandle<HashSet<String>>,
    selected_files: UseStateHandle<HashSet<String>>,
    context_menu_data: UseStateHandle<Option<ContextMenuData>>,
    toasts: UseStateHandle<Vec<Toast>>,
) -> Callback<MouseEvent> {
    Callback::from(move |e: MouseEvent| {
        e.prevent_default();

        // イベントエレメントから必要なデータを取得
        let x = e.client_x();
        let y = e.client_y();
        let Some(element) = downcast::<Element>(&e, &create_push_toast(toasts.clone())) else {
            return;
        };
        let is_dir = element
            .get_attribute("data-is-dir")
            .map(|v| v == "true")
            .unwrap_or(false);
        let path = element.get_attribute("data-path").unwrap_or_default();

        // ファイルを右クリックすることにより処理対象がこのファイルに限定されるため、
        // copy_files をクリアして selected_files にこのファイルをセット
        copy_files.set(Default::default());
        let mut new_value = HashSet::<String>::default();
        new_value.insert(path.clone());
        selected_files.set(new_value);

        context_menu_data.set(Some(ContextMenuData::new(
            Coordinate::new(x, y),
            is_dir,
            path,
        )));
    })
}

/// # Summary
///
/// ファイルのダブルクリックイベントハンドラを生成
pub fn create_file_anchor_double_click_handler(
    all_files: UseStateHandle<Vec<FileInfo>>,
    display_files: UseStateHandle<Vec<FileInfo>>,
    navigation_history: UseStateHandle<NavigationHistory>,
    toasts: UseStateHandle<Vec<Toast>>,
    click_timeout: Rc<RefCell<Option<Timeout>>>,
) -> Callback<MouseEvent> {
    Callback::from(move |e: MouseEvent| {
        e.prevent_default();

        let toasts = toasts.clone();
        let push_toast = create_push_toast(toasts);

        // 保留しているシングルクリックの処理をキャンセル
        if let Some(v) = click_timeout.borrow_mut().take() {
            v.cancel();
        }

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
/// ファイル名変更(マウス操作)のイベントハンドラを生成
pub fn create_file_textbox_blur_handler(
    all_files: UseStateHandle<Vec<FileInfo>>,
    display_files: UseStateHandle<Vec<FileInfo>>,
    navigation_history: UseStateHandle<NavigationHistory>,
    renaming_file: UseStateHandle<Option<String>>,
    selected_files: UseStateHandle<HashSet<String>>,
    toasts: UseStateHandle<Vec<Toast>>,
) -> Callback<FocusEvent> {
    Callback::from(move |e: FocusEvent| {
        let push_toast = create_push_toast(toasts.clone());
        let mut current_name: String = Default::default();

        if let Some(v) = (*renaming_file).clone() {
            current_name = Path::new(&v)
                .file_name()
                .and_then(|v| v.to_str())
                .unwrap_or_default()
                .to_string();
        }

        // 後続処理の成否に関わらず名称変更状態は解除
        renaming_file.set(None);
        // イベントエレメントから必要なデータを取得
        let Some(element) = downcast::<HtmlInputElement>(&e, &push_toast) else {
            return;
        };
        let new_name = element.value();
        let path = element.get_attribute("data-path").unwrap_or_default();

        if new_name.trim().is_empty() {
            user_error!("ファイル名を入力してください", &push_toast);
            return;
        }

        if current_name == new_name {
            warning!("ファイル名が変更されていません");
            return;
        }

        let all_files = all_files.clone();
        let display_files = display_files.clone();
        let nh = (*navigation_history).clone();
        let current_path = nh.current().to_string();
        let selected_files = selected_files.clone();
        spawn_local(async move {
            if tc_rename_file(&path, &new_name, push_toast.clone()).await {
                let file_infos = tc_read_dir(&current_path, push_toast).await;
                all_files.set(file_infos.clone());
                display_files.set(file_infos);

                let mut new_value = HashSet::<String>::new();
                new_value.insert(
                    Path::new(&current_path)
                        .join(&new_name)
                        .to_string_lossy()
                        .into_owned(),
                );
                // 選択しているファイルのファイル名を更新
                selected_files.set(new_value);
            }
        });
    })
}

/// # Summary
///
/// ファイル名変更(キー操作)のイベントハンドラを生成
pub fn create_file_textbox_keypress_handler(
    renaming_file: UseStateHandle<Option<String>>,
    toasts: UseStateHandle<Vec<Toast>>,
) -> Callback<KeyboardEvent> {
    Callback::from(move |e: KeyboardEvent| {
        // ファイル名の変更をキャンセル
        if e.key() == "Escape" {
            renaming_file.set(None);
            return;
        }

        let toasts = toasts.clone();

        // handle_file_textbox_blur で処理
        if e.key() == "Enter" {
            downcast::<HtmlInputElement>(&e, &create_push_toast(toasts)).inspect(|v| {
                let _ = v.blur();
            });
        }
    })
}
