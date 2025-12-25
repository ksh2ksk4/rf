use gloo_timers::callback::Timeout;
use gloo_timers::future::TimeoutFuture;
use rf_common::FileInfo;
use std::collections::HashSet;
use std::path::Path;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Element, HtmlInputElement, InputEvent};
use yew::prelude::*;

use crate::hooks::*;
use crate::models::*;
use crate::tauri_api::*;
use crate::utils::*;
use crate::{user_error, user_warning};

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

    // back ボタンクリックのイベントハンドラ
    let handle_back_click = {
        let display_files = display_files.clone();
        let navigation_history = navigation_history.clone();
        let push_toast = push_toast.clone();
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
    };

    // forward ボタンクリックのイベントハンドラ
    let handle_forward_click = {
        let display_files = display_files.clone();
        let navigation_history = navigation_history.clone();
        let push_toast = push_toast.clone();
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
    };

    // "go to parent dir" ボタンクリックのイベントハンドラ
    let handle_go_to_parent_dir_click = {
        let display_files = display_files.clone();
        let navigation_history = navigation_history.clone();
        let push_toast = push_toast.clone();
        Callback::from(move |_| {
            let display_files = display_files.clone();
            let navigation_history = navigation_history.clone();
            let push_toast = push_toast.clone();
            spawn_local(async move {
                let path =
                    tc_get_parent_dir(navigation_history.current(), push_toast.clone()).await;
                display_files.set(tc_read_dir(&path, push_toast).await);
                let mut nh = (*navigation_history).clone();
                nh.push(&path);
                navigation_history.set(nh);
            });
        })
    };

    // "select dir" ボタンクリックのイベントハンドラ
    let handle_select_dir_click = {
        let display_files = display_files.clone();
        let navigation_history = navigation_history.clone();
        let push_toast = push_toast.clone();
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
    };

    // reload ボタンクリックのイベントハンドラ
    let handle_reload_click = {
        let display_files = display_files.clone();
        let navigation_history = navigation_history.clone();
        let push_toast = push_toast.clone();
        Callback::from(move |_| {
            let display_files = display_files.clone();
            let nh = (*navigation_history).clone();
            let current_path = nh.current().to_string();
            let push_toast = push_toast.clone();
            spawn_local(async move {
                display_files.set(tc_read_dir(&current_path, push_toast).await);
            });
        })
    };

    // copy ボタンクリックのイベントハンドラ
    let handle_copy_click = {
        let copy_files = copy_files.clone();
        let selected_files = selected_files.clone();
        Callback::from(move |_| {
            copy_files.set((*selected_files).clone());
        })
    };

    // paste ボタンクリックのイベントハンドラ
    let handle_paste_click = {
        let copy_files = copy_files.clone();
        let display_files = display_files.clone();
        let navigation_history = navigation_history.clone();
        let push_toast = push_toast.clone();
        let selected_files = selected_files.clone();
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
    };

    // "delete files" ボタンクリックのイベントハンドラ
    let handle_delete_files_click = {
        let display_files = display_files.clone();
        let navigation_history = navigation_history.clone();
        let push_toast = push_toast.clone();
        let selected_files = selected_files.clone();
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
    };

    // フィルタ設定のイベントハンドラ
    let handle_filter_input = {
        let filter = filter.clone();
        let push_toast = push_toast.clone();
        Callback::from(move |e: InputEvent| {
            downcast::<HtmlInputElement>(&e, &push_toast).inspect(|v| {
                filter.set(v.value());
            });
        })
    };

    // チェックボックスクリックのイベントハンドラ
    let handle_checkbox_click = {
        let push_toast = push_toast.clone();
        let selected_files = selected_files.clone();
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
    };

    // ファイルクリックのイベントハンドラ
    let handle_file_click = {
        let click_timeout = click_timeout.clone();
        let push_toast = push_toast.clone();
        let renaming_file = renaming_file.clone();
        let selected_files = selected_files.clone();
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
    };

    // ファイル名変更(マウス操作)のイベントハンドラ
    let handle_file_rename_blur = {
        let display_files = display_files.clone();
        let navigation_history = navigation_history.clone();
        let push_toast = push_toast.clone();
        let renaming_file = renaming_file.clone();
        let selected_files = selected_files.clone();
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
    };

    // ファイル名変更(キー操作)のイベントハンドラ
    let handle_file_rename_keypress = {
        let push_toast = push_toast.clone();
        let renaming_file = renaming_file.clone();
        Callback::from(move |e: KeyboardEvent| {
            // ファイル名の変更をキャンセル
            if e.key() == "Escape" {
                renaming_file.set(None);
                return;
            }

            // handle_file_rename_blur で処理
            if e.key() == "Enter" {
                downcast::<HtmlInputElement>(&e, &push_toast).inspect(|v| {
                    let _ = v.blur();
                });
            }
        })
    };

    // ファイルダブルクリックのイベントハンドラ
    let handle_file_double_click = {
        let click_timeout = click_timeout.clone();
        let display_files = display_files.clone();
        let navigation_history = navigation_history.clone();
        let push_toast = push_toast.clone();
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
    };

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
                        onclick={handle_back_click}
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
                        onclick={handle_forward_click}
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
                        onclick={handle_go_to_parent_dir_click}
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
                        onclick={handle_select_dir_click}
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
                        onclick={handle_reload_click}
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
                        onclick={handle_copy_click}
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
                        onclick={handle_paste_click}
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
                        onclick={handle_delete_files_click}
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
                            oninput={handle_filter_input}
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
                                            onchange={handle_checkbox_click.clone()}
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
                                                    onblur={handle_file_rename_blur.clone()}
                                                    onkeypress={handle_file_rename_keypress.clone()}
                                                    data-path={f.path().clone()}
                                                />
                                            }
                                        } else {
                                            html! {
                                                <a
                                                    href="#"
                                                    onclick={handle_file_click.clone()}
                                                    ondblclick={handle_file_double_click.clone()}
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
