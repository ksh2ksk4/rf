use gloo_timers::callback::Timeout;
use gloo_timers::future::TimeoutFuture;
use rf_common::FileInfo;
use std::collections::HashSet;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, HtmlInputElement};
use yew::prelude::*;

use super::handlers::*;
use crate::shared::models::*;
use crate::shared::utils::*;

#[derive(PartialEq, Properties)]
pub struct MainProps {
    pub display_files: UseStateHandle<Vec<FileInfo>>,
    pub navigation_history: UseStateHandle<NavigationHistory>,
    pub selected_files: UseStateHandle<HashSet<String>>,
    pub toasts: UseStateHandle<Vec<Toast>>,
}

/// # Summary
///
/// メインを生成する
///
/// # Returns
///
/// `Html`: HTML
#[function_component(Main)]
pub fn main_component(props: &MainProps) -> Html {
    //
    // アプリ共有のステート
    //
    let display_files = &props.display_files;
    let navigation_history = &props.navigation_history;
    let selected_files = &props.selected_files;
    let toasts = &props.toasts;

    //
    // Main 固有のステート
    //
    // 名称変更中のファイル
    let renaming_file = use_state(|| Option::<String>::None);

    // シングルクリック処理用のキャンセラブルタイマー
    let click_timeout = use_mut_ref(|| None::<Timeout>);

    //
    // フック
    //
    {
        let renaming_file = renaming_file.clone();
        // ファイル名変更時にテキストボックスを focus & select
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

    //
    // イベントハンドラ
    //
    #[rustfmt::skip]
    let handle_file_checkbox_click = create_file_checkbox_click_handler(
        selected_files.clone(),
        toasts.clone(),
    );
    let handle_file_anchor_click = create_file_anchor_click_handler(
        renaming_file.clone(),
        selected_files.clone(),
        toasts.clone(),
        click_timeout.clone(),
    );
    let handle_file_anchor_double_click = create_file_anchor_double_click_handler(
        display_files.clone(),
        navigation_history.clone(),
        toasts.clone(),
        click_timeout.clone(),
    );
    let handle_file_textbox_blur = create_file_textbox_blur_handler(
        display_files.clone(),
        navigation_history.clone(),
        renaming_file.clone(),
        selected_files.clone(),
        toasts.clone(),
    );
    #[rustfmt::skip]
    let handle_file_textbox_keypress = create_file_textbox_keypress_handler(
        renaming_file.clone(),
        toasts.clone(),
    );

    html! {
        <main>
            <table class="file-list">
                <thead>
                    <tr>
                        <th>
                            <input
                                type="checkbox"
                                checked=false
                                aria-label="select all"
                            />
                        </th>
                        <th>{"name"}</th>
                        <th>{"size"}</th>
                        <th>{"created at"}</th>
                        <th>{"modified at"}</th>
                        <th>{"accessed at"}</th>
                    </tr>
                </thead>
                <tbody>
                    {for display_files.iter().map(|f| {
                        html! {
                            <tr class={if f.is_dir() {"dir"} else {"file"}}>
                                <td class="select-file">
                                    <input
                                        type="checkbox"
                                        checked={(*selected_files).contains(&f.path())}
                                        onchange={handle_file_checkbox_click.clone()}
                                        data-path={f.path().clone()}
                                        aria-label="select file"
                                    />
                                </td>
                                <td class="name">
                                    {if f.is_dir() {
                                        html! {<i class="line-start folder nf nf-fa-folder" />}
                                    } else {
                                        html! {<i class="line-start file nf nf-fa-file" />}
                                    }}
                                    {if (*renaming_file).as_ref().map(|v| v == &f.path()).unwrap_or(false) {
                                        html! {
                                            <input
                                                id="renaming_file"
                                                type="text"
                                                value={f.name().clone()}
                                                onblur={handle_file_textbox_blur.clone()}
                                                onkeypress={handle_file_textbox_keypress.clone()}
                                                data-path={f.path().clone()}
                                            />
                                        }
                                    } else {
                                        html! {
                                            <a
                                                href="#"
                                                onclick={handle_file_anchor_click.clone()}
                                                ondblclick={handle_file_anchor_double_click.clone()}
                                                data-is-dir={f.is_dir().to_string()}
                                                data-path={f.path().clone()}
                                            >
                                                {&f.name()}
                                            </a>
                                        }
                                    }}
                                </td>
                                <td class="size">{convert_file_size(f.size())}</td>
                                <td class="datetime">{&f.created()}</td>
                                <td class="datetime">{&f.modified()}</td>
                                <td class="datetime">{&f.accessed()}</td>
                            </tr>
                        }
                    })}
                </tbody>
            </table>
        </main>
    }
}
