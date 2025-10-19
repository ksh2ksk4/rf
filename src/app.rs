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
    size: u64,
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
            <table>
                <thead>
                    <tr>
                        <th>{"name"}</th>
                        <th>{"size"}</th>
                    </tr>
                </thead>
                <tbody>
                    {for files.iter().map(|f| {
                        let name = f.name.clone();

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
                            <tr>
                                <td>{name}</td>
                                <td>{size_string}</td>
                            </tr>
                        }
                    })}
                </tbody>
            </table>
        </main>
    }
}
