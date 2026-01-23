mod components;
mod shared;

use crate::components::app::fc::*;

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
