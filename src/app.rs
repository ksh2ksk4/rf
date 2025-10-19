use gloo_utils::format::JsValueSerdeExt;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

const INIT_DIR: &str = "/Users/ksh2ksk4/Downloads";
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

#[function_component(App)]
pub fn app() -> Html {
    let files = use_state(|| Vec::<FileInfo>::new());

    // 初回マウント時に実行されるフック
    {
        let files = files.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                let path = INIT_DIR;
                let args = JsValue::from_serde(&serde_json::json!({"path": path})).unwrap();
                files.set(invoke("read_dir", args).await.into_serde().unwrap());
            });

            || {}
        });
    }

    html! {
        <main class="container">
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
                        let size_string = format!("{size:.2} {unit}");

                        html! {
                            <tr class={if is_dir {"dir"} else {"file"}}>
                                <td class="name">
                                    {if is_dir {
                                        html! {
                                            <span class="icon">
                                                <i class="nf nf-cod-folder" />
                                            </span>
                                        }
                                    } else {
                                        html! {
                                            <span class="icon">
                                                <i class="nf nf-cod-file" />
                                            </span>
                                        }
                                    }}
                                    {name}
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
        </main>
    }
}
