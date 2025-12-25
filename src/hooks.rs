use gloo::events::EventListener;
use gloo_timers::future::TimeoutFuture;
use rf_common::FileInfo;
use std::collections::HashSet;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, HtmlInputElement};
use yew::prelude::*;

use crate::debug;
use crate::models::*;
use crate::tauri_api::*;
use crate::utils::*;

/// # Summary
///
/// 初回マウント時に実行されるカスタムフック
#[hook]
pub fn use_init(
    all_files: UseStateHandle<Vec<FileInfo>>,
    display_files: UseStateHandle<Vec<FileInfo>>,
    navigation_history: UseStateHandle<NavigationHistory>,
    push_toast: Callback<(ToastKind, String)>,
    header_ref: NodeRef,
    footer_ref: NodeRef,
) {
    use_effect_with((), move |_| {
        // ファイルリスト表示領域の高さを設定
        set_content_height(&header_ref, &footer_ref);
        // ウィンドウリサイズ時に再計算するよう設定
        let window = window().unwrap();
        let resize = EventListener::new(&window, "resize", move |_| {
            set_content_height(&header_ref, &footer_ref);
        });

        // ファイルリストを初期表示
        spawn_local(async move {
            //note これは "temporary value dropped while borrowed" エラーになる
            //let path = navigation_history.paths().first().unwrap();
            let paths = navigation_history.paths();
            let path = paths.first().unwrap();
            let file_infos = tc_read_dir(path, push_toast).await;
            all_files.set(file_infos.clone());
            display_files.set(file_infos);
        });

        move || {
            drop(resize);
        }
    });
}

/// # Summary
///
/// ステート更新時にログを出力するカスタムフック
#[allow(unused_variables)]
#[hook]
pub fn use_state_logger(
    all_files: UseStateHandle<Vec<FileInfo>>,
    copy_files: UseStateHandle<HashSet<String>>,
    display_files: UseStateHandle<Vec<FileInfo>>,
    filter: UseStateHandle<String>,
    navigation_history: UseStateHandle<NavigationHistory>,
    renaming_file: UseStateHandle<Option<String>>,
    selected_files: UseStateHandle<HashSet<String>>,
) {
    use_effect_with(
        (
            all_files,
            copy_files,
            display_files,
            filter,
            navigation_history,
            renaming_file,
            selected_files,
        ),
        move |(
            all_files,
            copy_files,
            display_files,
            filter,
            navigation_history,
            renaming_file,
            selected_files,
        )| {
            //debug!(all_files);
            debug!(copy_files);
            //debug!(display_files);
            //debug!(filter);
            //debug!(navigation_history);
            //debug!(renaming_file);
            debug!(renaming_file);
            debug!(selected_files);

            || {}
        },
    );
}

/// # Summary
///
/// フィルタ更新時にフィルタリングを実行するカスタムフック
#[hook]
pub fn use_filter_effect(
    all_files: UseStateHandle<Vec<FileInfo>>,
    display_files: UseStateHandle<Vec<FileInfo>>,
    filter: UseStateHandle<String>,
) {
    use_effect_with(filter, move |filter| {
        let query = (*filter).to_lowercase();

        if query.is_empty() {
            display_files.set((*all_files).clone());
        } else {
            display_files.set(
                (*all_files)
                    .iter()
                    .filter(|f| f.name().to_lowercase().contains(&query))
                    .cloned()
                    .collect::<Vec<FileInfo>>(),
            );
        }

        || {}
    });
}

/// # Summary
///
/// ファイル名変更時にテキストボックスを focus & select するカスタムフック
#[hook]
pub fn use_rename_focus(renaming_file: UseStateHandle<Option<String>>) {
    use_effect_with(renaming_file, move |renaming_file| {
        if let Some(_) = (*renaming_file).as_ref() {
            spawn_local(async move {
                // DOM が確実に更新されるのを待機
                TimeoutFuture::new(0).await;

                window()
                    .and_then(|v| v.document())
                    .and_then(|v| v.get_element_by_id("renaming_file"))
                    .and_then(|v| v.dyn_into::<HtmlInputElement>().ok())
                    .map(|v| {
                        let _ = v.focus();
                        v.select();
                    });
            });
        }

        || {}
    });
}
