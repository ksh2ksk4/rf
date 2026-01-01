use rf_common::FileInfo;
use std::collections::HashSet;
use yew::prelude::*;

use super::handlers::*;
use crate::debug;
use crate::shared::models::*;

#[derive(PartialEq, Properties)]
pub struct HeaderProps {
    pub all_files: UseStateHandle<Vec<FileInfo>>,
    pub display_files: UseStateHandle<Vec<FileInfo>>,
    pub navigation_history: UseStateHandle<NavigationHistory>,
    pub selected_files: UseStateHandle<HashSet<String>>,
    pub toasts: UseStateHandle<Vec<Toast>>,
    pub header_ref: NodeRef,
}

/// # Summary
///
/// ヘッダを生成する
///
/// # Returns
///
/// `Html`: HTML
#[function_component(Header)]
pub fn header_component(props: &HeaderProps) -> Html {
    //
    // アプリ共有のステート
    //
    let all_files = &props.all_files;
    let display_files = &props.display_files;
    let navigation_history = &props.navigation_history;
    let selected_files = &props.selected_files;
    let toasts = &props.toasts;

    // <header> を参照する NodeRef
    let header_ref = &props.header_ref;

    //
    // Header 固有のステート
    //
    // コピー対象のファイル
    let copy_files = use_state(|| HashSet::<String>::new());
    // ファイル名に対するフィルタ
    let filter = use_state(|| String::new());

    //
    // フック
    //
    #[cfg(debug_assertions)]
    {
        let copy_files = copy_files.clone();
        let filter = filter.clone();
        // ステート更新時にログを出力(デバッグ用)
        #[allow(unused_variables)]
        use_effect_with((copy_files, filter), move |(copy_files, filter)| {
            debug!(copy_files);
            debug!(filter);

            || {}
        });
    }
    {
        let all_files = all_files.clone();
        let display_files = display_files.clone();
        let filter = filter.clone();
        // フィルタ更新時にフィルタリングを実行
        use_effect_with(filter, move |filter| {
            let query = (*filter).to_lowercase();

            if query.is_empty() {
                display_files.set((*all_files).clone());
            } else {
                display_files.set(
                    (*all_files)
                        .iter()
                        .filter(|f| f.name().to_lowercase().contains(&query))
                        .cloned()
                        .collect::<Vec<FileInfo>>(),
                );
            }

            || {}
        });
    }

    //
    // イベントハンドラ
    //
    let handle_back_button_click = create_back_button_click_handler(
        display_files.clone(),
        navigation_history.clone(),
        toasts.clone(),
    );
    let handle_forward_button_click = create_forward_button_click_handler(
        display_files.clone(),
        navigation_history.clone(),
        toasts.clone(),
    );
    let handle_go_to_parent_dir_button_click = create_go_to_parent_dir_button_click_handler(
        display_files.clone(),
        navigation_history.clone(),
        toasts.clone(),
    );
    let handle_select_dir_button_click = create_select_dir_button_click_handler(
        display_files.clone(),
        navigation_history.clone(),
        toasts.clone(),
    );
    let handle_reload_button_click = create_reload_button_click_handler(
        display_files.clone(),
        navigation_history.clone(),
        toasts.clone(),
    );
    #[rustfmt::skip]
    let handle_copy_button_click = create_copy_button_click_handler(
        copy_files.clone(),
        selected_files.clone(),
    );
    let handle_paste_button_click = create_paste_button_click_handler(
        copy_files.clone(),
        display_files.clone(),
        navigation_history.clone(),
        selected_files.clone(),
        toasts.clone(),
    );
    let handle_delete_files_button_click = create_delete_files_button_click_handler(
        display_files.clone(),
        navigation_history.clone(),
        selected_files.clone(),
        toasts.clone(),
    );
    #[rustfmt::skip]
    let handle_filter_textbox_input = create_filter_textbox_input_handler(
        filter.clone(),
        toasts.clone(),
    );

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
