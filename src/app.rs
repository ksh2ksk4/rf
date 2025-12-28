use gloo_timers::callback::Timeout;
use gloo_timers::future::TimeoutFuture;
use rf_common::FileInfo;
use std::collections::HashSet;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::handlers::*;
use crate::hooks::*;
use crate::models::*;
use crate::utils::*;

/// # Summary
///
/// メインコンテンツを表示する
///
/// # Returns
///
/// `Html`: HTML
#[function_component(App)]
pub fn app() -> Html {
    // カレントディレクトリのすべてのファイル
    let all_files = use_state(|| Vec::<FileInfo>::new());
    // コピー対象のファイル
    let copy_files = use_state(|| HashSet::<String>::new());
    // ファイルリストに表示するファイル(カレントディレクトリのファイルをフィルタリングしたもの)
    let display_files = use_state(|| Vec::<FileInfo>::new());
    // ファイル名に対するフィルタ
    let filter = use_state(|| String::new());
    // ディレクトリの移動履歴
    let navigation_history = use_state(|| NavigationHistory::new());
    // 名称変更中のファイル
    let renaming_file = use_state(|| Option::<String>::None);
    // 選択されたファイル
    let selected_files = use_state(|| HashSet::<String>::new());
    // 表示待ちのトースト
    let toasts = use_state(|| Vec::<Toast>::new());

    // シングルクリック処理用のキャンセラブルタイマー
    let click_timeout = use_mut_ref(|| None::<Timeout>);

    // <header> を参照する NodeRef
    let header_ref = use_node_ref();
    // <footer> を参照する NodeRef
    let footer_ref = use_node_ref();

    // トーストを表示するコールバック
    let push_toast = {
        let toasts = toasts.clone();
        Callback::from(move |(kind, message): (ToastKind, String)| {
            let new_toast = Toast::new(kind, message);
            let next_id = Toast::next_id();
            // トーストを追加して再設定
            let mut v = (*toasts).clone();
            v.push(new_toast);
            toasts.set(v);
            let toasts = toasts.clone();
            spawn_local(async move {
                TimeoutFuture::new(Toast::DURATION).await;
                // 表示済のトーストを除去して再設定
                let mut rest = (*toasts).clone();
                rest.retain(|v| v.id() < next_id);
                toasts.set(rest);
            });
        })
    };

    // 初回マウント時に実行されるカスタムフック
    use_init(
        all_files.clone(),
        display_files.clone(),
        navigation_history.clone(),
        push_toast.clone(),
        header_ref.clone(),
        footer_ref.clone(),
    );
    // ステート更新時にログを出力するカスタムフック
    use_state_logger(
        all_files.clone(),
        copy_files.clone(),
        display_files.clone(),
        filter.clone(),
        navigation_history.clone(),
        renaming_file.clone(),
        selected_files.clone(),
    );
    // フィルタ更新時にフィルタリングを実行するカスタムフック
    use_filter_effect(all_files.clone(), display_files.clone(), filter.clone());
    // ファイル名変更時にテキストボックスを focus & select するカスタムフック
    use_rename_focus(renaming_file.clone());

    //
    // イベントハンドラ
    //
    let handle_back_button_click = create_back_button_click_handler(
        display_files.clone(),
        navigation_history.clone(),
        push_toast.clone(),
    );
    let handle_forward_button_click = create_forward_button_click_handler(
        display_files.clone(),
        navigation_history.clone(),
        push_toast.clone(),
    );
    let handle_go_to_parent_dir_button_click = create_go_to_parent_dir_button_click_handler(
        display_files.clone(),
        navigation_history.clone(),
        push_toast.clone(),
    );
    let handle_select_dir_button_click = create_select_dir_button_click_handler(
        display_files.clone(),
        navigation_history.clone(),
        push_toast.clone(),
    );
    let handle_reload_button_click = create_reload_button_click_handler(
        display_files.clone(),
        navigation_history.clone(),
        push_toast.clone(),
    );
    let handle_copy_button_click =
        create_copy_button_click_handler(copy_files.clone(), selected_files.clone());
    let handle_paste_button_click = create_paste_button_click_handler(
        copy_files.clone(),
        display_files.clone(),
        navigation_history.clone(),
        push_toast.clone(),
        selected_files.clone(),
    );
    let handle_delete_files_button_click = create_delete_files_button_click_handler(
        display_files.clone(),
        navigation_history.clone(),
        push_toast.clone(),
        selected_files.clone(),
    );
    let handle_filter_textbox_input =
        create_filter_textbox_input_handler(filter.clone(), push_toast.clone());
    let handle_file_checkbox_click =
        create_file_checkbox_click_handler(push_toast.clone(), selected_files.clone());
    let handle_file_anchor_click = create_file_anchor_click_handler(
        click_timeout.clone(),
        push_toast.clone(),
        renaming_file.clone(),
        selected_files.clone(),
    );
    let handle_file_textbox_blur = create_file_textbox_blur_handler(
        display_files.clone(),
        navigation_history.clone(),
        push_toast.clone(),
        renaming_file.clone(),
        selected_files.clone(),
    );
    let handle_file_textbox_keypress =
        create_file_textbox_keypress_handler(push_toast.clone(), renaming_file.clone());
    let handle_file_anchor_double_click = create_file_anchor_double_click_handler(
        click_timeout.clone(),
        display_files.clone(),
        navigation_history.clone(),
        push_toast.clone(),
    );

    html! {
        <div class="min-h-screen min-w-screen flex flex-col">
            <div class="toast-area">
                {for toasts.iter().map(|t| {
                    let toasts = toasts.clone();
                    let id = t.id();
                    let handle_close_click = Callback::from(move |_| {
                        let mut new_value = (*toasts).clone();
                        new_value.retain(|v| v.id() < id);
                        toasts.set(new_value);
                    });
                    html! {
                        <div class={classes!(
                            "toast-base",
                            match t.kind() {
                                ToastKind::Success => "toast-success",
                                ToastKind::Info => "toast-info",
                                ToastKind::Warning => "toast-warning",
                                ToastKind::Error => "toast-error",
                            }
                        )}>
                            <i
                                class={classes!(
                                    "mr-2",
                                    "select-none",
                                    "nf",
                                    match t.kind() {
                                        ToastKind::Success => "nf-fa-ok_sign",
                                        ToastKind::Info => "nf-fa-circle_info",
                                        ToastKind::Warning => "nf-fa-warning",
                                        ToastKind::Error => "nf-fa-triangle_exclamation",
                                    }
                                )}
                                aria-hidden="true"
                            />
                            <span class="flex-1">{t.message().clone()}</span>
                            <button
                                class="ml-3 opacity-80 hover:opacity-100"
                                onclick={handle_close_click}
                                aria-label="dismiss"
                            >
                                <i class="nf nf-fa-window_close" />
                            </button>
                        </div>
                    }
                })}
            </div>
            <header ref={header_ref}>
                <div class="toolbar">
                    <button
                        class="icon"
                        title="back"
                        aria-label="back"
                        onclick={handle_back_button_click}
                        disabled={!navigation_history.can_back()}
                    >
                        <i
                            class="nf nf-fa-circle_left"
                            aria-hidden="true"
                        />
                    </button>
                    <button
                        class="icon"
                        title="forward"
                        aria-label="forward"
                        onclick={handle_forward_button_click}
                        disabled={!navigation_history.can_forward()}
                    >
                        <i
                            class="nf nf-fa-circle_right"
                            aria-hidden="true"
                        />
                    </button>
                    <button
                        class="icon"
                        title="go to parent dir"
                        aria-label="go to parent dir"
                        onclick={handle_go_to_parent_dir_button_click}
                    >
                        <i
                            class="nf nf-fa-circle_up"
                            aria-hidden="true"
                        />
                    </button>
                    <button
                        class="icon"
                        title="select dir"
                        aria-label="select dir"
                        onclick={handle_select_dir_button_click}
                    >
                        <i
                            class="nf nf-fa-folder_open"
                            aria-hidden="true"
                        />
                    </button>
                    <button
                        class="icon"
                        title="reload"
                        aria-label="reload"
                        onclick={handle_reload_button_click}
                    >
                        <i
                            class="nf nf-md-reload"
                            aria-hidden="true"
                        />
                    </button>
                    <button
                        class="icon"
                        title="copy"
                        aria-label="copy"
                        onclick={handle_copy_button_click}
                        disabled={selected_files.is_empty()}
                    >
                        <i
                            class="nf nf-fa-copy"
                            aria-hidden="true"
                        />
                    </button>
                    <button
                        class="icon"
                        title="paste"
                        aria-label="paste"
                        onclick={handle_paste_button_click}
                        disabled={copy_files.is_empty()}
                    >
                        <i
                            class="nf nf-fa-paste"
                            aria-hidden="true"
                        />
                    </button>
                    <button
                        class="icon"
                        title="delete files"
                        aria-label="delete files"
                        onclick={handle_delete_files_button_click}
                        disabled={selected_files.is_empty()}
                    >
                        <i
                            class="nf nf-fa-trash"
                            aria-hidden="true"
                        />
                    </button>
                    <div class="filter">
                        <i
                            class="nf nf-fa-filter"
                            aria-hidden="true"
                        />
                        <input
                            class="text-base"
                            oninput={handle_filter_textbox_input}
                            placeholder="filter files"
                            type="search"
                            value={(*filter).clone()}
                        />
                    </div>
                    <div class="search">
                        <i
                            class="nf nf-fa-search"
                            aria-hidden="true"
                        />
                        <input
                            class="text-base"
                            placeholder="search files"
                            type="search"
                        />
                    </div>
                </div>
            </header>
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
            <footer ref={footer_ref.clone()}>
                <div>{navigation_history.current()}</div>
            </footer>
        </div>
    }
}
