use yew::prelude::*;

use crate::components::context_menu::fc::*;

/// # Summary
///
/// アプリの任意部分のクリックイベントハンドラを生成
///
/// 表示中のコンテキストメニューを非表示にする
pub fn create_app_click_handler(
    context_menu: UseStateHandle<Option<ContextMenuData>>,
) -> Callback<MouseEvent> {
    Callback::from(move |_e: MouseEvent| {
        context_menu.set(None);
    })
}
