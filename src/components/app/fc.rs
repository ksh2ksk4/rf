use gloo::events::EventListener;
use rf_common::FileInfo;
use std::collections::HashSet;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, KeyboardEvent};
use yew::prelude::*;

use super::handlers::*;
use crate::components::context_menu::fc::*;
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
    // Shift キーの押下状態
    let shift_key_pressed = use_state(|| bool::default());
    // コンテキストメニュー(ファイルを右クリックしたときに表示するメニュー)のデータ
    // メニュー非表示対応のため <main> ではなく <app> で定義
    let context_menu_data = use_state(|| Option::<ContextMenuData>::default());
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
        let shift_key_pressed = shift_key_pressed.clone();
        let context_menu_data = context_menu_data.clone();
        let toasts = toasts.clone();
        // ステート更新時にログを出力(デバッグ用)
        #[allow(unused_variables)]
        use_effect_with(
            (
                all_files,
                display_files,
                navigation_history,
                selected_files,
                shift_key_pressed,
                context_menu_data,
                toasts,
            ),
            move |(
                all_files,
                display_files,
                navigation_history,
                selected_files,
                shift_key_pressed,
                context_menu_data,
                toasts,
            )| {
                debug!(all_files);
                debug!(display_files);
                debug!(navigation_history);
                debug!(selected_files);
                debug!(shift_key_pressed);
                debug!(context_menu_data);
                debug!(toasts);

                || {}
            },
        );
    }
    {
        let all_files = all_files.clone();
        let display_files = display_files.clone();
        let navigation_history = navigation_history.clone();
        let shift_key_pressed = shift_key_pressed.clone();
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

            //
            // Shift キーの押下状態を監視
            //
            let keydown = {
                let shift_key_pressed = shift_key_pressed.clone();
                EventListener::new(&window, "keydown", move |e| {
                    if let Some(v) = e.dyn_ref::<KeyboardEvent>() {
                        if v.key() == "Shift" {
                            shift_key_pressed.set(true);
                        }
                    }
                })
            };
            let keyup = {
                let shift_key_pressed = shift_key_pressed.clone();
                EventListener::new(&window, "keyup", move |e| {
                    if let Some(v) = e.dyn_ref::<KeyboardEvent>() {
                        if v.key() == "Shift" {
                            shift_key_pressed.set(false);
                        }
                    }
                })
            };

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
                drop(keydown);
                drop(keyup);
            }
        });
    }

    //
    // イベントハンドラ
    //
    #[rustfmt::skip]
    let handle_app_click = create_app_click_handler(
        context_menu_data.clone(),
    );

    html! {
        <div
            id="app"
            onclick={handle_app_click}
        >
            <Header
                all_files={all_files.clone()}
                display_files={display_files.clone()}
                navigation_history={navigation_history.clone()}
                selected_files={selected_files.clone()}
                shift_key_pressed={shift_key_pressed.clone()}
                toasts={toasts.clone()}
                header_ref={header_ref}
            />
            <Main
                all_files={all_files.clone()}
                display_files={display_files}
                navigation_history={navigation_history.clone()}
                selected_files={selected_files}
                context_menu_data={context_menu_data}
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
