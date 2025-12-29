use std::collections::HashSet;
use yew::prelude::*;

use crate::models::NavigationHistory;

pub fn build_header(
    header_ref: NodeRef,
    copy_files: UseStateHandle<HashSet<String>>,
    filter: UseStateHandle<String>,
    navigation_history: UseStateHandle<NavigationHistory>,
    selected_files: UseStateHandle<HashSet<String>>,
    handle_back_button_click: Callback<MouseEvent>,
    handle_forward_button_click: Callback<MouseEvent>,
    handle_go_to_parent_dir_button_click: Callback<MouseEvent>,
    handle_select_dir_button_click: Callback<MouseEvent>,
    handle_reload_button_click: Callback<MouseEvent>,
    handle_copy_button_click: Callback<MouseEvent>,
    handle_paste_button_click: Callback<MouseEvent>,
    handle_delete_files_button_click: Callback<MouseEvent>,
    handle_filter_textbox_input: Callback<InputEvent>,
) -> Html {
    html! {
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
    }
}
