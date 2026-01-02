use gloo::events::EventListener;
use rf_common::FileInfo;
use std::collections::HashSet;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;

use crate::components::footer::fc::*;
use crate::components::header::fc::*;
use crate::components::main::fc::*;
use crate::components::toast_area::fc::*;
use crate::debug;
use crate::shared::models::*;
use crate::shared::tauri_api::*;
use crate::shared::utils::*;

/// # Summary
///
/// メインコンテンツを生成する
///
/// # Returns
///
/// `Html`: HTML
#[function_component(App)]
pub fn app_component() -> Html {
    //
    // アプリ共有のステート
    //
    // カレントディレクトリのすべてのファイル
    let all_files = use_state(|| Vec::<FileInfo>::default());
    // ファイルリストに表示するファイル(カレントディレクトリのファイルをフィルタリングしたもの)
    let display_files = use_state(|| Vec::<FileInfo>::default());
    // ディレクトリの移動履歴
    let navigation_history = use_state(|| NavigationHistory::default());
    // 選択されたファイル
    let selected_files = use_state(|| HashSet::<String>::default());
    // 表示待ちのトースト
    let toasts = use_state(|| Vec::<Toast>::default());

    // <header> を参照する NodeRef
    let header_ref = use_node_ref();
    // <footer> を参照する NodeRef
    let footer_ref = use_node_ref();

    //
    // フック
    //
    #[cfg(debug_assertions)]
    {
        let all_files = all_files.clone();
        let display_files = display_files.clone();
        let navigation_history = navigation_history.clone();
        let selected_files = selected_files.clone();
        let toasts = toasts.clone();
        // ステート更新時にログを出力(デバッグ用)
        #[allow(unused_variables)]
        use_effect_with(
            (
                all_files,
                display_files,
                navigation_history,
                selected_files,
                toasts,
            ),
            move |(all_files, display_files, navigation_history, selected_files, toasts)| {
                debug!(all_files);
                debug!(display_files);
                debug!(navigation_history);
                debug!(selected_files);
                debug!(toasts);

                || {}
            },
        );
    }
    {
        let all_files = all_files.clone();
        let display_files = display_files.clone();
        let navigation_history = navigation_history.clone();
        let toasts = toasts.clone();
        let header_ref = header_ref.clone();
        let footer_ref = footer_ref.clone();
        // 初回マウント時に実行する処理
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
                let file_infos = tc_read_dir(path, create_push_toast(toasts)).await;
                all_files.set(file_infos.clone());
                display_files.set(file_infos);
            });

            move || {
                drop(resize);
            }
        });
    }

    html! {
        <div id="app">
            <Header
                all_files={all_files.clone()}
                display_files={display_files.clone()}
                navigation_history={navigation_history.clone()}
                selected_files={selected_files.clone()}
                toasts={toasts.clone()}
                header_ref={header_ref}
            />
            <Main
                all_files={all_files.clone()}
                display_files={display_files}
                navigation_history={navigation_history.clone()}
                selected_files={selected_files}
                toasts={toasts.clone()}
            />
            <Footer
                navigation_history={navigation_history}
                footer_ref={footer_ref}
            />
            <ToastArea toasts={toasts} />
        </div>
    }
}
