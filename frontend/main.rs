mod components;
mod shared;

use crate::components::app::fc::*;
use crate::shared::tauri_api::*;
use wasm_bindgen_futures::spawn_local;

fn main() {
    console_error_panic_hook::set_once();
    spawn_local(async {
        let init_dir = tc_get_init_dir().await;
        yew::Renderer::<App>::with_props(AppProps { init_dir }).render();
    });
}
