use yew::prelude::*;

use crate::models::*;

#[derive(PartialEq, Properties)]
pub struct FooterProps {
    pub navigation_history: UseStateHandle<NavigationHistory>,
    pub footer_ref: NodeRef,
}

/// # Summary
///
/// フッタを生成する
///
/// # Returns
///
/// `Html`: HTML
#[function_component(Footer)]
pub fn footer_component(props: &FooterProps) -> Html {
    //
    // アプリ共有のステート
    //
    let navigation_history = &props.navigation_history;

    // <footer> を参照する NodeRef
    let footer_ref = &props.footer_ref;

    html! {
        <footer ref={footer_ref}>
            <div>{navigation_history.current()}</div>
        </footer>
    }
}
