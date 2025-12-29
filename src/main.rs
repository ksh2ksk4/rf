mod app;
mod components;
mod handlers;
mod macros;
mod models;
mod tauri_api;
mod utils;

use crate::app::App;

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
