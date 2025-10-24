use gloo_utils::format::JsValueSerdeExt;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use yew::prelude::*;

const INIT_PATH: &str = "/Users/ksh2ksk4/Downloads";
const UNITS: [&str; 5] = ["B", "KiB", "MiB", "GiB", "TiB"];

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
struct FileInfo {
    name: String,
    path: String,
    is_dir: bool,
    is_file: bool,
    is_symlink: bool,
    // Unix only
    is_block_device: bool,
    // Unix only
    is_char_device: bool,
    // Unix only
    is_fifo: bool,
    // Unix only
    is_socket: bool,
    size: u64,
    readonly: bool,
    // Unix only
    mode: u32,
    accessed: String,
    created: String,
    modified: String,
}

#[derive(Clone, Debug, PartialEq)]
struct NavigationHistory {
    index: usize,
    paths: Vec<String>,
}

impl NavigationHistory {
    pub fn new() -> Self {
        Self {
            index: 0,
            paths: vec![INIT_PATH.to_string()],
        }
    }

    pub fn can_back(&self) -> bool {
        self.index > 0
    }

    pub fn can_forward(&self) -> bool {
        self.index + 1 < self.paths.len()
    }

    pub fn current(&self) -> &str {
        &self.paths[self.index]
    }

    pub fn back(&mut self) -> Option<String> {
        if !self.can_back() {
            None
        } else {
            self.index -= 1;
            Some(self.current().to_string())
        }
    }

    pub fn forward(&mut self) -> Option<String> {
        if !self.can_forward() {
            None
        } else {
            self.index += 1;
            Some(self.current().to_string())
        }
    }

    pub fn push(&mut self, path: &str) {
        if self.index + 1 < self.paths.len() {
            // 最新の移動履歴ではない場合
            self.paths.truncate(self.index + 1);
        }

        self.paths.push(path.to_string());
        self.index = self.paths.len() - 1;
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let navigation_history = use_state(|| NavigationHistory::new());
    let files = use_state(|| Vec::<FileInfo>::new());

    // 初回マウント時に実行されるフック
    {
        let navigation_history = navigation_history.clone();
        let files = files.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                let path = navigation_history.paths[0].clone();
                let args = JsValue::from_serde(&serde_json::json!({"path": path})).unwrap();
                files.set(invoke("read_dir", args).await.into_serde().unwrap());
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

    // Back ボタンクリックのイベントハンドラ
    let handle_back_click = {
        let navigation_history = navigation_history.clone();
        let files = files.clone();
        Callback::from(move |_| {
            let mut nh = (*navigation_history).clone();
            let path = nh.back().unwrap_or(INIT_PATH.to_string());
            navigation_history.set(nh);
            let files = files.clone();
            spawn_local(async move {
                let args = JsValue::from_serde(&serde_json::json!({"path": path})).unwrap();
                files.set(invoke("read_dir", args).await.into_serde().unwrap());
            });
        })
    };

    // Forward ボタンクリックのイベントハンドラ
    let handle_forward_click = {
        let navigation_history = navigation_history.clone();
        let files = files.clone();
        Callback::from(move |_| {
            let mut nh = (*navigation_history).clone();
            let path = nh.forward().unwrap_or(nh.current().to_string());
            navigation_history.set(nh);
            let files = files.clone();
            spawn_local(async move {
                let args = JsValue::from_serde(&serde_json::json!({"path": path})).unwrap();
                files.set(invoke("read_dir", args).await.into_serde().unwrap());
            });
        })
    };

    // "Select Dir" ボタンクリックのイベントハンドラ
    let handle_select_dir_click = {
        let files = files.clone();
        Callback::from(move |_| {
            let files = files.clone();
            spawn_local(async move {
                let folder = invoke("select_dir", JsValue::NULL)
                    .await
                    .as_string()
                    .unwrap();
                let args = JsValue::from_serde(&serde_json::json!({"path": folder})).unwrap();
                files.set(invoke("read_dir", args).await.into_serde().unwrap());
            });
        })
    };

    html! {
        <main class="container">
            <div class="flex gap-2">
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
            </div>
            <div>
                <table class="file-list">
                    <caption class="caption-top">
                      {navigation_history.current()}
                    </caption>
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
                                let path = f.path.clone();
                                Callback::from(move |e: MouseEvent| {
                                    e.prevent_default();

                                    if !is_dir {
                                        return;
                                    }

                                    let mut nh = (*navigation_history).clone();
                                    nh.push(&path);
                                    navigation_history.set(nh);

                                    let files = files.clone();
                                    let path = path.clone();
                                    spawn_local(async move {
                                        let args = JsValue::from_serde(&serde_json::json!({"path": path})).unwrap();
                                        files.set(invoke("read_dir", args).await.into_serde().unwrap());
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
                                            html! {<i class="head nf nf-fa-folder" />}
                                        } else {
                                            html! {<i class="head nf nf-fa-file" />}
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
    }
}
