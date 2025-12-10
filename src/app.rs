use gloo_timers::future::TimeoutFuture;
use gloo_utils::format::JsValueSerdeExt;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{console, Element, HtmlInputElement, InputEvent};
use yew::prelude::*;

// トーストを表示する時間(ms)
const TOAST_DURATION: u32 = 5000;
// 初期表示パス
const INIT_PATH: &str = "/Users/ksh2ksk4/Downloads";
// TAURI コマンド
const TAURI_COMMAND_DELETE_FILES: &str = "delete_files";
const TAURI_COMMAND_GET_PARENT_DIR: &str = "get_parent_dir";
const TAURI_COMMAND_OPEN_FILE: &str = "open_file";
const TAURI_COMMAND_READ_DIR: &str = "read_dir";
const TAURI_COMMAND_RENAME_FILE: &str = "rename_file";
const TAURI_COMMAND_SELECT_DIR: &str = "select_dir";

#[wasm_bindgen]
extern "C" {
    /**
     * エラーが発生しない TAURI コマンドを実行する場合
     */
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
    #[wasm_bindgen(js_name = invoke, js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke_no_args(cmd: &str) -> JsValue;
    /**
     * エラーが発生する可能性のある TAURI コマンドを実行する場合
     */
    #[wasm_bindgen(catch, js_name = invoke, js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke_r(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(catch, js_name = invoke, js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke_r_no_args(cmd: &str) -> Result<JsValue, JsValue>;
}

/// # Summary
///
/// ファイルに関するデータ
///
/// # Fields
///
/// - `name`: 名前
/// - `path`: パス(フルパス)
/// - `is_dir`: ディレクトリかどうかを表すフラグ
/// - `is_file`: ファイルかどうかを表すフラグ
/// - `is_symlink`: シンボリックリンクかどうかを表すフラグ
/// - `is_block_device`: ブロックデバイスかどうかを表すフラグ(UNIX only)
/// - `is_char_device`: キャラクタデバイスかどうかを表すフラグ(UNIX only)
/// - `is_fifo`: FIFO かどうかを表すフラグ(UNIX only)
/// - `is_socket`: ソケットかどうかを表すフラグ(UNIX only)
/// - `size`: サイズ
/// - `readonly`: 読取専用かどうかを表すフラグ
/// - `mode`: モード(UNIX only)
/// - `accessed`: アクセス日時
/// - `created`: 作成日時
/// - `modified`: 更新日時
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
struct FileInfo {
    name: String,
    path: String,
    is_dir: bool,
    is_file: bool,
    is_symlink: bool,
    is_block_device: bool,
    is_char_device: bool,
    is_fifo: bool,
    is_socket: bool,
    size: u64,
    readonly: bool,
    mode: u32,
    accessed: String,
    created: String,
    modified: String,
}

/// # Summary
///
/// 表示履歴に関するデータ
///
/// # Fields
///
/// - `index`: 表示中のパスを指し示すインデックス
/// - `paths`: 表示したパスのリスト
#[derive(Clone, Debug, PartialEq)]
struct NavigationHistory {
    index: usize,
    paths: Vec<String>,
}

impl NavigationHistory {
    /// # Summary
    ///
    /// インスタンスを生成
    ///
    /// # Returns
    ///
    /// - `Self`: インスタンス
    pub fn new() -> Self {
        Self {
            index: 0,
            paths: vec![INIT_PATH.to_string()],
        }
    }

    /// # Summary
    ///
    /// 一つ前のパスに戻れるかチェック
    ///
    /// # Returns
    ///
    /// - `bool`: 一つ前のパスに戻れるかどうか
    pub fn can_back(&self) -> bool {
        self.index > 0
    }

    /// # Summary
    ///
    /// 一つ後のパスに進めるかチェック
    ///
    /// # Returns
    ///
    /// - `bool`: 一つ後のパスに進めるかどうか
    pub fn can_forward(&self) -> bool {
        self.index + 1 < self.paths.len()
    }

    /// # Summary
    ///
    /// 現在のパスを返す
    ///
    /// # Returns
    ///
    /// - `&str`: パス
    pub fn current(&self) -> &str {
        &self.paths[self.index]
    }

    /// # Summary
    ///
    /// 一つ前のパスに戻る
    ///
    /// # Returns
    ///
    /// - `Some(String)`: 一つ前のパス
    /// - `None`: 前のパスがない場合
    pub fn back(&mut self) -> Option<String> {
        if !self.can_back() {
            None
        } else {
            self.index -= 1;
            Some(self.current().to_string())
        }
    }

    /// # Summary
    ///
    /// 一つ後のパスに進む
    ///
    /// # Returns
    ///
    /// - `Some(String)`: 一つ後のパス
    /// - `None`: 後のパスがない場合
    pub fn forward(&mut self) -> Option<String> {
        if !self.can_forward() {
            None
        } else {
            self.index += 1;
            Some(self.current().to_string())
        }
    }

    /// # Summary
    ///
    /// 履歴にパスを追加
    ///
    /// # Arguments
    ///
    /// - `path`: パス
    pub fn push(&mut self, path: &str) {
        if self.index + 1 < self.paths.len() {
            // 最新の移動履歴ではない場合
            self.paths.truncate(self.index + 1);
        }

        self.paths.push(path.to_string());
        self.index = self.paths.len() - 1;
    }
}

/// # Summary
///
/// トーストの種類
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
enum ToastKind {
    Success,
    Info,
    Warning,
    Error,
}

/// # Summary
///
/// 個々のトーストに関するデータ
///
/// # Fields
///
/// - `id`: ID
/// - `kind`: 種類
/// - `message`: メッセージ
#[derive(Clone, Debug, PartialEq)]
struct Toast {
    id: usize,
    kind: ToastKind,
    message: String,
}

/// # Summary
///
/// メインコンテンツを表示する
///
/// # Returns
///
/// `Html`: HTML
#[function_component(App)]
pub fn app() -> Html {
    // ディレクトリの移動履歴
    let navigation_history = use_state(|| NavigationHistory::new());
    // カレントディレクトリのすべてのファイル
    let all_files = use_state(|| Vec::<FileInfo>::new());
    // ファイルリストに表示するファイル(カレントディレクトリのファイルをフィルタリングしたもの)
    let display_files = use_state(|| Vec::<FileInfo>::new());
    // ファイル名に対するフィルタ
    let filter = use_state(|| String::new());
    // 名称変更中のファイル
    let renaming_file = use_state(|| Option::<String>::None);
    // 選択されたファイル
    let selected_files = use_state(|| HashSet::<String>::new());

    let toasts = use_state(|| Vec::<Toast>::new());
    let next_toast_id = use_state(|| 1_usize);
    let push_toast = {
        let toasts = toasts.clone();
        let next_toast_id = next_toast_id.clone();
        Callback::from(move |(kind, message): (ToastKind, String)| {
            let mut v = (*toasts).clone();
            let id = *next_toast_id;
            v.push(Toast { id, kind, message });
            toasts.set(v);
            next_toast_id.set(id + 1);

            let toasts = toasts.clone();
            spawn_local(async move {
                TimeoutFuture::new(TOAST_DURATION).await;
                let mut new_value = (*toasts).clone();
                new_value.retain(|v| v.id != id);
                toasts.set(new_value);
            });
        })
    };

    // 初回マウント時に実行されるフック
    {
        let navigation_history = navigation_history.clone();
        let all_files = all_files.clone();
        let display_files = display_files.clone();
        let push_toast = push_toast.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                let path = navigation_history.paths.first().unwrap();
                let file_infos = read_dir(path, push_toast).await;
                all_files.set(file_infos.clone());
                display_files.set(file_infos);
            });

            || {}
        });
    }

    // ステート更新時にログを出力するフック
    {
        let navigation_history = navigation_history.clone();
        let all_files = all_files.clone();
        let display_files = display_files.clone();
        let filter = filter.clone();
        let renaming_file = renaming_file.clone();
        let selected_files = selected_files.clone();
        #[allow(unused_variables)]
        use_effect_with(
            (
                navigation_history,
                all_files,
                display_files,
                filter,
                renaming_file,
                selected_files,
            ),
            move |(
                navigation_history,
                all_files,
                display_files,
                filter,
                renaming_file,
                selected_files,
            )| {
                //console::info_1(&format!("navigation_history: {navigation_history:?}").into());
                //console::info_1(&format!("all_files: {all_files:?}").into());
                //console::info_1(&format!("display_files: {display_files:?}").into());
                //console::info_1(&format!("filter: {filter:?}").into());
                console::info_1(&format!("renaming_file: {renaming_file:?}").into());
                console::info_1(&format!("selected_files: {selected_files:?}").into());

                || {}
            },
        );
    }

    // フィルタ更新時にフィルタリングを実行するフック
    {
        let all_files = all_files.clone();
        let display_files = display_files.clone();
        let filter = filter.clone();
        //let push_toast = push_toast.clone();
        use_effect_with(filter, move |filter| {
            let query = (*filter).to_lowercase();

            if query.is_empty() {
                display_files.set((*all_files).clone());
            } else {
                display_files.set(
                    (*all_files)
                        .iter()
                        .filter(|f| f.name.to_lowercase().contains(&query))
                        .cloned()
                        .collect::<Vec<FileInfo>>(),
                );
            }

            || {}
        });
    }

    // back ボタンクリックのイベントハンドラ
    let handle_back_click = {
        let navigation_history = navigation_history.clone();
        let display_files = display_files.clone();
        let push_toast = push_toast.clone();
        Callback::from(move |_| {
            let mut nh = (*navigation_history).clone();
            let path = nh.back().unwrap_or(INIT_PATH.to_string());
            navigation_history.set(nh);
            let display_files = display_files.clone();
            let push_toast = push_toast.clone();
            spawn_local(async move {
                display_files.set(read_dir(&path, push_toast).await);
            });
        })
    };

    // forward ボタンクリックのイベントハンドラ
    let handle_forward_click = {
        let navigation_history = navigation_history.clone();
        let display_files = display_files.clone();
        let push_toast = push_toast.clone();
        Callback::from(move |_| {
            let mut nh = (*navigation_history).clone();
            let path = nh.forward().unwrap_or(nh.current().to_string());
            navigation_history.set(nh);
            let display_files = display_files.clone();
            let push_toast = push_toast.clone();
            spawn_local(async move {
                display_files.set(read_dir(&path, push_toast).await);
            });
        })
    };

    // "go to parent dir" ボタンクリックのイベントハンドラ
    let handle_go_to_parent_dir_click = {
        let navigation_history = navigation_history.clone();
        let display_files = display_files.clone();
        let push_toast = push_toast.clone();
        Callback::from(move |_| {
            let navigation_history = navigation_history.clone();
            let display_files = display_files.clone();
            let push_toast = push_toast.clone();
            spawn_local(async move {
                let path = get_parent_dir(navigation_history.current(), push_toast.clone()).await;
                display_files.set(read_dir(&path, push_toast).await);
                let mut nh = (*navigation_history).clone();
                nh.push(&path);
                navigation_history.set(nh);
            });
        })
    };

    // "select dir" ボタンクリックのイベントハンドラ
    let handle_select_dir_click = {
        let navigation_history = navigation_history.clone();
        let display_files = display_files.clone();
        let push_toast = push_toast.clone();
        Callback::from(move |_| {
            let navigation_history = navigation_history.clone();
            let display_files = display_files.clone();
            let push_toast = push_toast.clone();
            spawn_local(async move {
                let path = select_dir().await;
                display_files.set(read_dir(&path, push_toast).await);
                let mut nh = (*navigation_history).clone();
                nh.push(&path);
                navigation_history.set(nh);
            });
        })
    };

    // "reload" ボタンクリックのイベントハンドラ
    let handle_reload_click = {
        let navigation_history = navigation_history.clone();
        let display_files = display_files.clone();
        let push_toast = push_toast.clone();
        Callback::from(move |_| {
            let nh = (*navigation_history).clone();
            let current_path = nh.current().to_string();
            let display_files = display_files.clone();
            let push_toast = push_toast.clone();
            spawn_local(async move {
                display_files.set(read_dir(&current_path, push_toast).await);
            });
        })
    };

    // "delete files" ボタンクリックのイベントハンドラ
    let handle_delete_files_click = {
        let navigation_history = navigation_history.clone();
        let display_files = display_files.clone();
        let selected_files = selected_files.clone();
        let push_toast = push_toast.clone();
        Callback::from(move |_| {
            let nh = (*navigation_history).clone();
            let current_path = nh.current().to_string();
            let display_files = display_files.clone();
            let selected_files = selected_files.clone();
            let paths: Vec<String> = (*selected_files).iter().cloned().collect();
            let push_toast = push_toast.clone();
            spawn_local(async move {
                if delete_files(paths, push_toast.clone()).await {
                    display_files.set(read_dir(&current_path, push_toast).await);
                    // 選択状態をクリア
                    selected_files.set(HashSet::new());
                }
            });
        })
    };

    // フィルタ設定のイベントハンドラ
    let handle_filter_input = {
        let filter = filter.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            filter.set(input.value());
        })
    };

    // チェックボックスクリックのイベントハンドラ
    let handle_checkbox_click = {
        let selected_files = selected_files.clone();
        Callback::from(move |e: Event| {
            let element = match e
                .target()
                .and_then(|v| v.dyn_into::<HtmlInputElement>().ok())
            {
                Some(v) => v,
                None => return,
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
        let renaming_file = renaming_file.clone();
        let selected_files = selected_files.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            let element = match e.target().and_then(|v| v.dyn_into::<Element>().ok()) {
                Some(v) => v,
                None => return,
            };
            let path = element.get_attribute("data-path").unwrap_or_default();
            let mut new_value = HashSet::<String>::new();
            new_value.insert(path.clone());

            if (*selected_files).clone() == new_value {
                // 選択しているファイルを再度クリックした場合
                renaming_file.set(Some(path));
            }

            selected_files.set(new_value);
        })
    };

    // ファイル名変更(マウス操作)のイベントハンドラ
    let handle_file_rename_blur = {
        let navigation_history = navigation_history.clone();
        let display_files = display_files.clone();
        let renaming_file = renaming_file.clone();
        let push_toast = push_toast.clone();
        Callback::from(move |e: FocusEvent| {
            // 後続処理の成否に関わらず名称変更状態は解除
            renaming_file.set(None);

            let (new_name, path) = match e
                .target()
                .and_then(|v| v.dyn_into::<HtmlInputElement>().ok())
            {
                Some(v) => (v.value(), v.get_attribute("data-path").unwrap_or_default()),
                None => return,
            };

            if new_name.trim().is_empty() {
                return;
            }

            let nh = (*navigation_history).clone();
            let current_path = nh.current().to_string();
            let display_files = display_files.clone();
            let push_toast = push_toast.clone();
            spawn_local(async move {
                if rename_file(&path, &new_name, push_toast.clone()).await {
                    display_files.set(read_dir(&current_path, push_toast).await);
                }
            });
        })
    };

    // ファイル名変更(キー操作)のイベントハンドラ
    let handle_file_rename_keypress = {
        let renaming_file = renaming_file.clone();
        Callback::from(move |e: KeyboardEvent| {
            // ファイル名の変更をキャンセル
            if e.key() == "Escape" {
                renaming_file.set(None);
                return;
            }

            // handle_file_rename_blur で処理
            if e.key() == "Enter" {
                if let Some(element) = e
                    .target()
                    .and_then(|v| v.dyn_into::<HtmlInputElement>().ok())
                {
                    let _ = element.blur();
                }
            }
        })
    };

    // ファイルダブルクリックのイベントハンドラ
    let handle_file_double_click = {
        let navigation_history = navigation_history.clone();
        let display_files = display_files.clone();
        let push_toast = push_toast.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            //note Yew のイベントハンドラはキャプチャリングが有効なので `current_target()` は <a> ではなく <body> になる
            let element = match e.target().and_then(|v| v.dyn_into::<Element>().ok()) {
                Some(v) => {
                    //let tag_name = v.tag_name();
                    //console::info_1(&format!("{tag_name:?}").into());
                    v
                }
                None => return,
            };
            let is_dir = element
                .get_attribute("data-is-dir")
                .map(|v| v == "true")
                .unwrap_or(false);
            let path = element.get_attribute("data-path").unwrap_or_default();

            if !is_dir {
                let push_toast = push_toast.clone();
                let path = path.clone();
                spawn_local(async move {
                    open_file(path, push_toast).await;
                });

                return;
            }

            let mut nh = (*navigation_history).clone();
            nh.push(&path);
            navigation_history.set(nh);

            let display_files = display_files.clone();
            let push_toast = push_toast.clone();
            let path = path.clone();
            spawn_local(async move {
                display_files.set(read_dir(&path, push_toast).await);
            });
        })
    };

    html! {
        <div class="min-h-screen min-w-screen flex flex-col">
            <div class="toast-area">
                {for toasts.iter().map(|t| {
                    let toasts = toasts.clone();
                    let id = t.id;
                    let handle_close_click = Callback::from(move |_| {
                        let mut new_value = (*toasts).clone();
                        new_value.retain(|v| v.id != id);
                        toasts.set(new_value);
                    });
                    html! {
                        <div class={classes!(
                            "toast-base",
                            match t.kind {
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
                                    match t.kind {
                                        ToastKind::Success => "nf-fa-ok_sign",
                                        ToastKind::Info => "nf-fa-circle_info",
                                        ToastKind::Warning => "nf-fa-warning",
                                        ToastKind::Error => "nf-fa-triangle_exclamation",
                                    }
                                )}
                                aria-hidden="true"
                            />
                            <span class="flex-1">{t.message.clone()}</span>
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
            <header>
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
            <main class="flex-1 overflow-auto">
                <div class="overflow-auto max-h-[80vh]">
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
                                    <tr class={if f.is_dir {"dir"} else {"file"}}>
                                        <td class="select-file">
                                            <input
                                                type="checkbox"
                                                checked={(*selected_files).contains(&f.path)}
                                                onchange={handle_checkbox_click.clone()}
                                                data-path={f.path.clone()}
                                                aria-label="select file"
                                            />
                                        </td>
                                        <td class="name">
                                            {if f.is_dir {
                                                html! {<i class="line-start folder nf nf-fa-folder" />}
                                            } else {
                                                html! {<i class="line-start file nf nf-fa-file" />}
                                            }}
                                            {if (*renaming_file).as_ref().map(|v| v == &f.path).unwrap_or(false) {
                                                html! {
                                                    <input
                                                        type="text"
                                                        value={f.name.clone()}
                                                        onblur={handle_file_rename_blur.clone()}
                                                        onkeypress={handle_file_rename_keypress.clone()}
                                                        data-path={f.path.clone()}
                                                    />
                                                }
                                            } else {
                                                html! {
                                                    <a
                                                        href="#"
                                                        onclick={handle_file_click.clone()}
                                                        ondblclick={handle_file_double_click.clone()}
                                                        data-is-dir={f.is_dir.to_string()}
                                                        data-path={f.path.clone()}
                                                    >
                                                        {&f.name}
                                                    </a>
                                                }
                                            }}
                                        </td>
                                        <td class="size">{convert_file_size(f.size)}</td>
                                        <td class="datetime">{&f.created}</td>
                                        <td class="datetime">{&f.modified}</td>
                                        <td class="datetime">{&f.accessed}</td>
                                    </tr>
                                }
                            })}
                        </tbody>
                    </table>
                </div>
            </main>
            <footer>
                <div>{navigation_history.current()}</div>
            </footer>
        </div>
    }
}

/// # Summary
///
/// 指定したファイルを削除する
///
/// # Arguments
///
/// - `paths`: 対象ファイルのパス
/// - `push_toast`: エラーメッセージ表示用のトースト
///
/// # Returns
///
/// - `bool`: 正常に削除したかどうか
async fn delete_files(paths: Vec<String>, push_toast: Callback<(ToastKind, String)>) -> bool {
    let args = match JsValue::from_serde(&serde_json::json!({"paths": paths})) {
        Ok(v) => v,
        Err(e) => {
            console::error_1(&format!("{e:?}").into());
            push_toast.emit((ToastKind::Error, format!("{e:?}")));
            return false;
        }
    };
    match invoke_r(TAURI_COMMAND_DELETE_FILES, args).await {
        Ok(_) => true,
        Err(e) => {
            console::error_1(&e);
            push_toast.emit((ToastKind::Error, format!("{e:?}")));
            false
        }
    }
}

/// # Summary
///
/// 指定したディレクトリの親ディレクトリのパスを取得する
///
/// # Arguments
///
/// - `path`: 対象ディレクトリのパス
/// - `push_toast`: エラーメッセージ表示用のトースト
///
/// # Returns
///
/// - `String`: 親ディレクトリのパス(エラーの場合は空文字)
async fn get_parent_dir(path: &str, push_toast: Callback<(ToastKind, String)>) -> String {
    let args = match JsValue::from_serde(&serde_json::json!({"path": path})) {
        Ok(v) => v,
        Err(e) => {
            console::error_1(&format!("{e:?}").into());
            push_toast.emit((ToastKind::Error, format!("{e:?}")));
            return String::new();
        }
    };
    invoke(TAURI_COMMAND_GET_PARENT_DIR, args)
        .await
        .as_string()
        .unwrap()
}

/// # Summary
///
/// 指定したファイルをデフォルトアプリでオープンする
///
/// # Arguments
///
/// - `path`: 対象ファイルのパス
/// - `push_toast`: エラーメッセージ表示用のトースト
async fn open_file(path: String, push_toast: Callback<(ToastKind, String)>) {
    let args = match JsValue::from_serde(&serde_json::json!({"path": path})) {
        Ok(v) => v,
        Err(e) => {
            console::error_1(&format!("{e:?}").into());
            push_toast.emit((ToastKind::Error, format!("{e:?}")));
            return;
        }
    };
    let _ = invoke_r(TAURI_COMMAND_OPEN_FILE, args)
        .await
        .inspect_err(|e| {
            console::error_1(e);
            push_toast.emit((ToastKind::Error, format!("{e:?}")));
        });
}

/// # Summary
///
/// 指定したディレクトリのファイルリストを取得する
///
/// # Arguments
///
/// - `path`: 対象ディレクトリのパス
/// - `push_toast`: エラーメッセージ表示用のトースト
///
/// # Returns
///
/// - `Vec<FileInfo>`: ファイルリスト(エラーの場合は空のリスト)
async fn read_dir(path: &String, push_toast: Callback<(ToastKind, String)>) -> Vec<FileInfo> {
    let args = match JsValue::from_serde(&serde_json::json!({"path": path})) {
        Ok(v) => v,
        Err(e) => {
            console::error_1(&format!("{e:?}").into());
            push_toast.emit((ToastKind::Error, format!("{e:?}")));
            return vec![];
        }
    };
    match invoke_r(TAURI_COMMAND_READ_DIR, args).await {
        Ok(v) => match v.into_serde::<Vec<FileInfo>>() {
            Ok(v) => v,
            Err(e) => {
                console::error_1(&format!("{e:?}").into());
                push_toast.emit((ToastKind::Error, format!("{e:?}")));
                vec![]
            }
        },
        Err(e) => {
            console::error_1(&e);
            push_toast.emit((ToastKind::Error, format!("{e:?}")));
            vec![]
        }
    }
}

/// # Summary
///
/// 指定したファイルをリネームする
///
/// # Arguments
///
/// - `path`: 対象ファイルのパス
/// - `new_name`: 変更後のファイル名
/// - `push_toast`: エラーメッセージ表示用のトースト
///
/// # Returns
///
/// - `bool`: リネームできたかどうか
async fn rename_file(
    path: &String,
    new_name: &String,
    push_toast: Callback<(ToastKind, String)>,
) -> bool {
    let args = match JsValue::from_serde(&serde_json::json!({"path": path, "new_name": new_name})) {
        Ok(v) => v,
        Err(e) => {
            console::error_1(&format!("{e:?}").into());
            push_toast.emit((ToastKind::Error, format!("{e:?}")));
            return false;
        }
    };
    match invoke_r(TAURI_COMMAND_RENAME_FILE, args).await {
        Ok(_) => true,
        Err(e) => {
            console::error_1(&e);
            push_toast.emit((ToastKind::Error, format!("{e:?}")));
            false
        }
    }
}

/// # Summary
///
/// ファイル選択ダイアログを表示してディレクトリを選択する
///
/// # Returns
///
/// - `String`: 選択したディレクトリのパス
async fn select_dir() -> String {
    invoke_no_args(TAURI_COMMAND_SELECT_DIR)
        .await
        .as_string()
        .unwrap()
}

/// # Summary
///
/// ファイルサイズを分かり易い文字列に変換する
///
/// # Arguments
///
/// - `file_size`: ファイルサイズ
///
/// # Returns
///
/// - `String`: ファイルサイズ文字列
fn convert_file_size(file_size: u64) -> String {
    // ファイルサイズの単位
    const UNITS: [&str; 5] = ["B", "KiB", "MiB", "GiB", "TiB"];

    let mut size = file_size as f64;
    let mut i: usize = 0;
    let (size, i) = loop {
        if size < 1024.0 {
            break (size, i);
        }

        size /= 1024.0;
        i += 1;
    };
    let unit = UNITS[i];
    // 小数点第二位で丸める
    let size_rounded = (size * 100.0).round() / 100.0;

    // 小数部がほぼ 0 かどうかチェック
    if size_rounded.fract() < f64::EPSILON {
        format!("{size:.0} {unit}")
    } else {
        format!("{size:.2} {unit}")
    }
}
