use yew::prelude::*;

use crate::models::*;

pub fn build_footer(
    footer_ref: NodeRef,
    navigation_history: UseStateHandle<NavigationHistory>,
) -> Html {
    html! {
        <footer ref={footer_ref}>
            <div>{navigation_history.current()}</div>
        </footer>
    }
}
