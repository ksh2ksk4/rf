use gloo_timers::callback::Timeout;
use rf_common::FileInfo;
use std::cell::RefCell;
use std::collections::HashSet;
use std::path::Path;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Element, Event, HtmlInputElement, InputEvent};
use yew::prelude::*;

use crate::models::*;
use crate::tauri_api::*;
use crate::utils::*;
use crate::{user_error, user_warning};

/// # Summary
///
/// back ボタンのクリックイベントハンドラを生成
pub fn create_back_button_click_handler(
    display_files: UseStateHandle<Vec<FileInfo>>,
    navigation_history: UseStateHandle<NavigationHistory>,
    push_toast: Callback<(ToastKind, String)>,
) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        let display_files = display_files.clone();
        let mut nh = (*navigation_history).clone();
        let path = nh.back();
        navigation_history.set(nh);
        let push_toast = push_toast.clone();
        spawn_local(async move {
            display_files.set(tc_read_dir(&path, push_toast).await);
        });
    })
}

/// # Summary
///
/// forward ボタンのクリックイベントハンドラを生成
pub fn create_forward_button_click_handler(
    display_files: UseStateHandle<Vec<FileInfo>>,
    navigation_history: UseStateHandle<NavigationHistory>,
    push_toast: Callback<(ToastKind, String)>,
) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        let display_files = display_files.clone();
        let mut nh = (*navigation_history).clone();
        let path = nh.forward();
        navigation_history.set(nh);
        let push_toast = push_toast.clone();
        spawn_local(async move {
            display_files.set(tc_read_dir(&path, push_toast).await);
        });
    })
}

/// # Summary
///
/// "go to parent dir" ボタンのクリックイベントハンドラを生成
pub fn create_go_to_parent_dir_button_click_handler(
    display_files: UseStateHandle<Vec<FileInfo>>,
    navigation_history: UseStateHandle<NavigationHistory>,
    push_toast: Callback<(ToastKind, String)>,
) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        let display_files = display_files.clone();
        let navigation_history = navigation_history.clone();
        let push_toast = push_toast.clone();
        spawn_local(async move {
            let path = tc_get_parent_dir(navigation_history.current(), push_toast.clone()).await;
            display_files.set(tc_read_dir(&path, push_toast).await);
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
    display_files: UseStateHandle<Vec<FileInfo>>,
    navigation_history: UseStateHandle<NavigationHistory>,
    push_toast: Callback<(ToastKind, String)>,
) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        let display_files = display_files.clone();
        let navigation_history = navigation_history.clone();
        let push_toast = push_toast.clone();
        spawn_local(async move {
            let path = tc_select_dir().await;
            display_files.set(tc_read_dir(&path, push_toast).await);
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
    display_files: UseStateHandle<Vec<FileInfo>>,
    navigation_history: UseStateHandle<NavigationHistory>,
    push_toast: Callback<(ToastKind, String)>,
) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        let display_files = display_files.clone();
        let nh = (*navigation_history).clone();
        let current_path = nh.current().to_string();
        let push_toast = push_toast.clone();
        spawn_local(async move {
            display_files.set(tc_read_dir(&current_path, push_toast).await);
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
    copy_files: UseStateHandle<HashSet<String>>,
    display_files: UseStateHandle<Vec<FileInfo>>,
    navigation_history: UseStateHandle<NavigationHistory>,
    push_toast: Callback<(ToastKind, String)>,
    selected_files: UseStateHandle<HashSet<String>>,
) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        let copy_files = copy_files.clone();
        let display_files = display_files.clone();
        let nh = (*navigation_history).clone();
        let current_path = nh.current().to_string();
        let push_toast = push_toast.clone();
        let selected_files = selected_files.clone();
        let paths: Vec<String> = (*copy_files).iter().cloned().collect();
        spawn_local(async move {
            tc_copy_files(paths, &current_path, push_toast.clone()).await;
            display_files.set(tc_read_dir(&current_path, push_toast).await);
            copy_files.set(Default::default());
            selected_files.set(Default::default());
        });
    })
}

/// # Summary
///
/// "delete files" ボタンのクリックイベントハンドラを生成
pub fn create_delete_files_button_click_handler(
    display_files: UseStateHandle<Vec<FileInfo>>,
    navigation_history: UseStateHandle<NavigationHistory>,
    push_toast: Callback<(ToastKind, String)>,
    selected_files: UseStateHandle<HashSet<String>>,
) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        let display_files = display_files.clone();
        let nh = (*navigation_history).clone();
        let current_path = nh.current().to_string();
        let push_toast = push_toast.clone();
        let selected_files = selected_files.clone();
        let paths: Vec<String> = (*selected_files).iter().cloned().collect();
        spawn_local(async move {
            if tc_delete_files(paths, push_toast.clone()).await {
                display_files.set(tc_read_dir(&current_path, push_toast).await);
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
    push_toast: Callback<(ToastKind, String)>,
) -> Callback<InputEvent> {
    Callback::from(move |e: InputEvent| {
        downcast::<HtmlInputElement>(&e, &push_toast).inspect(|v| {
            filter.set(v.value());
        });
    })
}

/// # Summary
///
/// ファイルチェックボックスのクリックイベントハンドラを生成
pub fn create_file_checkbox_click_handler(
    push_toast: Callback<(ToastKind, String)>,
    selected_files: UseStateHandle<HashSet<String>>,
) -> Callback<Event> {
    Callback::from(move |e: Event| {
        // イベントエレメントから必要なデータを取得
        let Some(element) = downcast::<HtmlInputElement>(&e, &push_toast) else {
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
    click_timeout: Rc<RefCell<Option<Timeout>>>,
    push_toast: Callback<(ToastKind, String)>,
    renaming_file: UseStateHandle<Option<String>>,
    selected_files: UseStateHandle<HashSet<String>>,
) -> Callback<MouseEvent> {
    Callback::from(move |e: MouseEvent| {
        e.prevent_default();

        // イベントエレメントから必要なデータを取得
        let Some(element) = downcast::<Element>(&e, &push_toast) else {
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
/// ファイル名変更(マウス操作)のイベントハンドラを生成
pub fn create_file_textbox_blur_handler(
    display_files: UseStateHandle<Vec<FileInfo>>,
    navigation_history: UseStateHandle<NavigationHistory>,
    push_toast: Callback<(ToastKind, String)>,
    renaming_file: UseStateHandle<Option<String>>,
    selected_files: UseStateHandle<HashSet<String>>,
) -> Callback<FocusEvent> {
    Callback::from(move |e: FocusEvent| {
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
            user_error!("ファイル名を入力してください", push_toast);
            return;
        }

        if current_name == new_name {
            user_warning!("ファイル名が変更されていません");
            return;
        }

        let display_files = display_files.clone();
        let nh = (*navigation_history).clone();
        let current_path = nh.current().to_string();
        let push_toast = push_toast.clone();
        let selected_files = selected_files.clone();
        spawn_local(async move {
            if tc_rename_file(&path, &new_name, push_toast.clone()).await {
                display_files.set(tc_read_dir(&current_path, push_toast).await);

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
    push_toast: Callback<(ToastKind, String)>,
    renaming_file: UseStateHandle<Option<String>>,
) -> Callback<KeyboardEvent> {
    Callback::from(move |e: KeyboardEvent| {
        // ファイル名の変更をキャンセル
        if e.key() == "Escape" {
            renaming_file.set(None);
            return;
        }

        // handle_file_textbox_blur で処理
        if e.key() == "Enter" {
            downcast::<HtmlInputElement>(&e, &push_toast).inspect(|v| {
                let _ = v.blur();
            });
        }
    })
}

/// # Summary
///
/// ファイルのダブルクリックイベントハンドラを生成
pub fn create_file_anchor_double_click_handler(
    click_timeout: Rc<RefCell<Option<Timeout>>>,
    display_files: UseStateHandle<Vec<FileInfo>>,
    navigation_history: UseStateHandle<NavigationHistory>,
    push_toast: Callback<(ToastKind, String)>,
) -> Callback<MouseEvent> {
    Callback::from(move |e: MouseEvent| {
        e.prevent_default();

        // 保留しているシングルクリックの処理をキャンセル
        if let Some(v) = click_timeout.borrow_mut().take() {
            v.cancel();
        }

        //note Yew のイベントハンドラはキャプチャリングが有効なので `current_target()` は <a> ではなく <body> になる
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
            let push_toast = push_toast.clone();
            spawn_local(async move {
                tc_open_file(path, push_toast).await;
            });
            return;
        }

        let display_files = display_files.clone();
        let mut nh = (*navigation_history).clone();
        nh.push(&path);
        navigation_history.set(nh);
        let path = path.clone();
        let push_toast = push_toast.clone();
        spawn_local(async move {
            display_files.set(tc_read_dir(&path, push_toast).await);
        });
    })
}
