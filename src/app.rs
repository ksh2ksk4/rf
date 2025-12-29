use gloo_timers::callback::Timeout;
use rf_common::FileInfo;
use std::collections::HashSet;
use yew::prelude::*;

use crate::components::div_toast_area::*;
use crate::components::footer::*;
use crate::components::header::*;
use crate::components::main::*;
use crate::handlers::*;
use crate::hooks::*;
use crate::models::*;
use crate::utils::*;

/// # Summary
///
/// メインコンテンツを表示する
///
/// # Returns
///
/// `Html`: HTML
#[function_component(App)]
pub fn app() -> Html {
    // カレントディレクトリのすべてのファイル
    let all_files = use_state(|| Vec::<FileInfo>::new());
    // コピー対象のファイル
    let copy_files = use_state(|| HashSet::<String>::new());
    // ファイルリストに表示するファイル(カレントディレクトリのファイルをフィルタリングしたもの)
    let display_files = use_state(|| Vec::<FileInfo>::new());
    // ファイル名に対するフィルタ
    let filter = use_state(|| String::new());
    // ディレクトリの移動履歴
    let navigation_history = use_state(|| NavigationHistory::new());
    // 名称変更中のファイル
    let renaming_file = use_state(|| Option::<String>::None);
    // 選択されたファイル
    let selected_files = use_state(|| HashSet::<String>::new());
    // 表示待ちのトースト
    let toasts = use_state(|| Vec::<Toast>::new());

    // シングルクリック処理用のキャンセラブルタイマー
    let click_timeout = use_mut_ref(|| None::<Timeout>);

    // <header> を参照する NodeRef
    let header_ref = use_node_ref();
    // <footer> を参照する NodeRef
    let footer_ref = use_node_ref();

    //
    // コールバック
    //
    let push_toast = create_push_toast(toasts.clone());

    //
    // カスタムフック
    //
    use_init(
        all_files.clone(),
        display_files.clone(),
        navigation_history.clone(),
        push_toast.clone(),
        header_ref.clone(),
        footer_ref.clone(),
    );
    use_state_logger(
        all_files.clone(),
        copy_files.clone(),
        display_files.clone(),
        filter.clone(),
        navigation_history.clone(),
        renaming_file.clone(),
        selected_files.clone(),
    );
    use_filter_effect(all_files.clone(), display_files.clone(), filter.clone());
    use_rename_focus(renaming_file.clone());

    //
    // イベントハンドラ
    //
    let handle_back_button_click = create_back_button_click_handler(
        display_files.clone(),
        navigation_history.clone(),
        push_toast.clone(),
    );
    let handle_forward_button_click = create_forward_button_click_handler(
        display_files.clone(),
        navigation_history.clone(),
        push_toast.clone(),
    );
    let handle_go_to_parent_dir_button_click = create_go_to_parent_dir_button_click_handler(
        display_files.clone(),
        navigation_history.clone(),
        push_toast.clone(),
    );
    let handle_select_dir_button_click = create_select_dir_button_click_handler(
        display_files.clone(),
        navigation_history.clone(),
        push_toast.clone(),
    );
    let handle_reload_button_click = create_reload_button_click_handler(
        display_files.clone(),
        navigation_history.clone(),
        push_toast.clone(),
    );
    let handle_copy_button_click =
        create_copy_button_click_handler(copy_files.clone(), selected_files.clone());
    let handle_paste_button_click = create_paste_button_click_handler(
        copy_files.clone(),
        display_files.clone(),
        navigation_history.clone(),
        push_toast.clone(),
        selected_files.clone(),
    );
    let handle_delete_files_button_click = create_delete_files_button_click_handler(
        display_files.clone(),
        navigation_history.clone(),
        push_toast.clone(),
        selected_files.clone(),
    );
    let handle_filter_textbox_input =
        create_filter_textbox_input_handler(filter.clone(), push_toast.clone());
    let handle_file_checkbox_click =
        create_file_checkbox_click_handler(push_toast.clone(), selected_files.clone());
    let handle_file_anchor_click = create_file_anchor_click_handler(
        click_timeout.clone(),
        push_toast.clone(),
        renaming_file.clone(),
        selected_files.clone(),
    );
    let handle_file_textbox_blur = create_file_textbox_blur_handler(
        display_files.clone(),
        navigation_history.clone(),
        push_toast.clone(),
        renaming_file.clone(),
        selected_files.clone(),
    );
    let handle_file_textbox_keypress =
        create_file_textbox_keypress_handler(push_toast.clone(), renaming_file.clone());
    let handle_file_anchor_double_click = create_file_anchor_double_click_handler(
        click_timeout.clone(),
        display_files.clone(),
        navigation_history.clone(),
        push_toast.clone(),
    );

    html! {
        <div class="min-h-screen min-w-screen flex flex-col">
            {build_div_toast_area(toasts)}
            {build_header(
                header_ref,
                copy_files,
                filter,
                navigation_history.clone(),
                selected_files.clone(),
                handle_back_button_click,
                handle_forward_button_click,
                handle_go_to_parent_dir_button_click,
                handle_select_dir_button_click,
                handle_reload_button_click,
                handle_copy_button_click,
                handle_paste_button_click,
                handle_delete_files_button_click,
                handle_filter_textbox_input,
            )}
            {build_main(
                display_files,
                selected_files,
                renaming_file,
                handle_file_checkbox_click,
                handle_file_textbox_blur,
                handle_file_textbox_keypress,
                handle_file_anchor_click,
                handle_file_anchor_double_click,
            )}
            {build_footer(footer_ref, navigation_history)}
        </div>
    }
}
