use gloo_timers::future::TimeoutFuture;
use gloo_utils::format::JsValueSerdeExt;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use yew::prelude::*;

// トーストを表示する時間(ms)
const TOAST_DURATION: u32 = 5000;
// 初期表示パス
const INIT_PATH: &str = "/Users/ksh2ksk4/Downloads";
// ファイルサイズの単位
const UNITS: [&str; 5] = ["B", "KiB", "MiB", "GiB", "TiB"];

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
    /// - `path`: パス(&str)
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
    let navigation_history = use_state(|| NavigationHistory::new());
    let files = use_state(|| Vec::<FileInfo>::new());
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
        let files = files.clone();
        let push_toast = push_toast.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                let path = navigation_history.paths[0].clone();
                let args = JsValue::from_serde(&serde_json::json!({"path": path})).unwrap();
                invoke_r("read_dir", args)
                    .await
                    .map(|v| files.set(v.into_serde().unwrap()))
                    .unwrap_or_else(|e| {
                        console::error_1(&e);
                        push_toast.emit((ToastKind::Error, format!("{e:?}")));
                    });
            });

            || {}
        });
    }

    // state の値が変化したときに実行されるフック
    {
        let navigation_history = navigation_history.clone();
        let files = files.clone();
        #[allow(unused_variables)]
        use_effect_with(
            (navigation_history, files),
            move |(navigation_history, files)| {
                console::info_1(&format!("navigation_history: {navigation_history:?}").into());
                //console::info_1(&format!("files: {files:?}").into());

                || {}
            },
        );
    }

    // back ボタンクリックのイベントハンドラ
    let handle_back_click = {
        let navigation_history = navigation_history.clone();
        let files = files.clone();
        let push_toast = push_toast.clone();
        Callback::from(move |_| {
            let mut nh = (*navigation_history).clone();
            let path = nh.back().unwrap_or(INIT_PATH.to_string());
            navigation_history.set(nh);
            let files = files.clone();
            let push_toast = push_toast.clone();
            spawn_local(async move {
                let args = JsValue::from_serde(&serde_json::json!({"path": path})).unwrap();
                invoke_r("read_dir", args)
                    .await
                    .map(|v| files.set(v.into_serde().unwrap()))
                    .unwrap_or_else(|e| {
                        console::error_1(&e);
                        push_toast.emit((ToastKind::Error, format!("{e:?}")));
                    });
            });
        })
    };

    // forward ボタンクリックのイベントハンドラ
    let handle_forward_click = {
        let navigation_history = navigation_history.clone();
        let files = files.clone();
        let push_toast = push_toast.clone();
        Callback::from(move |_| {
            let mut nh = (*navigation_history).clone();
            let path = nh.forward().unwrap_or(nh.current().to_string());
            navigation_history.set(nh);
            let files = files.clone();
            let push_toast = push_toast.clone();
            spawn_local(async move {
                let args = JsValue::from_serde(&serde_json::json!({"path": path})).unwrap();
                invoke_r("read_dir", args)
                    .await
                    .map(|v| files.set(v.into_serde().unwrap()))
                    .unwrap_or_else(|e| {
                        console::error_1(&e);
                        push_toast.emit((ToastKind::Error, format!("{e:?}")));
                    });
            });
        })
    };

    // "select dir" ボタンクリックのイベントハンドラ
    let handle_select_dir_click = {
        let navigation_history = navigation_history.clone();
        let files = files.clone();
        let push_toast = push_toast.clone();
        Callback::from(move |_| {
            let navigation_history = navigation_history.clone();
            let files = files.clone();
            let push_toast = push_toast.clone();
            spawn_local(async move {
                let path = invoke_no_args("select_dir").await.as_string().unwrap();
                let args = JsValue::from_serde(&serde_json::json!({"path": path})).unwrap();
                invoke_r("read_dir", args)
                    .await
                    .map(|v| {
                        files.set(v.into_serde().unwrap());
                        push_toast.emit((ToastKind::Success, "read_dir() succeeded".to_string()));
                    })
                    .unwrap_or_else(|e| {
                        console::error_1(&e);
                        push_toast.emit((ToastKind::Error, format!("{e:?}")));
                    });
                let mut nh = (*navigation_history).clone();
                nh.push(&path);
                navigation_history.set(nh);
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
                                        ToastKind::Error => "nf-fa-triangle_exclamation"
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
                        title="delete files"
                        aria-label="delete files"
                        //onclick={handle_delete_files_click}
                    >
                        <i
                            class="nf nf-fa-trash"
                            aria-hidden="true"
                        />
                    </button>
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
                                <th>{"name"}</th>
                                <th>{"size"}</th>
                                <th>{"created at"}</th>
                                <th>{"modified at"}</th>
                                <th>{"accessed at"}</th>
                            </tr>
                        </thead>
                        <tbody>
                            {for files.iter().map(|f| {
                                let is_dir = f.is_dir;

                                let handle_dir_click = {
                                    let navigation_history = navigation_history.clone();
                                    let files = files.clone();
                                    let push_toast = push_toast.clone();
                                    let path = f.path.clone();
                                    Callback::from(move |e: MouseEvent| {
                                        e.prevent_default();

                                        if !is_dir {
                                            let push_toast = push_toast.clone();
                                            let path = path.clone();
                                            spawn_local(async move {
                                                let args = JsValue::from_serde(
                                                    &serde_json::json!({"path": path})
                                                ).unwrap();
                                                invoke_r("open_file", args)
                                                    .await
                                                    .map(|_| console::info_1(&"open_file() succeeded".into()))
                                                    .unwrap_or_else(|e| {
                                                        console::error_1(&e);
                                                        push_toast.emit((ToastKind::Error, format!("{e:?}")));
                                                    });
                                            });

                                            return;
                                        }

                                        let mut nh = (*navigation_history).clone();
                                        nh.push(&path);
                                        navigation_history.set(nh);

                                        let files = files.clone();
                                        let push_toast = push_toast.clone();
                                        let path = path.clone();
                                        spawn_local(async move {
                                            let args = JsValue::from_serde(
                                                &serde_json::json!({"path": path})
                                            ).unwrap();
                                            invoke_r("read_dir", args)
                                                .await
                                                .map(|v| files.set(v.into_serde().unwrap()))
                                                .unwrap_or_else(|e| {
                                                    console::error_1(&e);
                                                    push_toast.emit((ToastKind::Error, format!("{e:?}")));
                                                });
                                        });
                                    })
                                };

                                let name = f.name.clone();
                                let created = f.created.clone();
                                let modified = f.modified.clone();
                                let accessed = f.accessed.clone();

                                let mut size = f.size as f64;
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
                                let size_string = if size_rounded.fract() < f64::EPSILON {
                                    format!("{size:.0} {unit}")
                                } else {
                                    format!("{size:.2} {unit}")
                                };

                                html! {
                                    <tr class={if is_dir {"dir"} else {"file"}}>
                                        <td class="name">
                                            {if is_dir {
                                                html! {<i class="line-start folder nf nf-fa-folder" />}
                                            } else {
                                                html! {<i class="line-start file nf nf-fa-file" />}
                                            }}
                                            <a
                                                href="#"
                                                onclick={handle_dir_click}
                                            >
                                                {name}
                                            </a>
                                        </td>
                                        <td class="size">{size_string}</td>
                                        <td class="datetime">{created}</td>
                                        <td class="datetime">{modified}</td>
                                        <td class="datetime">{accessed}</td>
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
