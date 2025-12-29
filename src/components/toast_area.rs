use yew::prelude::*;

use crate::models::*;

#[derive(PartialEq, Properties)]
pub struct ToastAreaProps {
    pub toasts: UseStateHandle<Vec<Toast>>,
}

/// # Summary
///
/// トースト表示領域を生成する
///
/// # Returns
///
/// `Html`: HTML
#[function_component(ToastArea)]
pub fn toast_area_component(props: &ToastAreaProps) -> Html {
    //
    // アプリ共有のステート
    //
    let toasts = &props.toasts;

    html! {
        <div class="toast-area">
            {for toasts.iter().map(|t| {
                let toasts = toasts.clone();
                let id = t.id();
                let handle_close_click = Callback::from(move |_| {
                    let mut new_value = (*toasts).clone();
                    new_value.retain(|v| v.id() < id);
                    toasts.set(new_value);
                });
                html! {
                    <div class={classes!(
                        "toast-base",
                        match t.kind() {
                            ToastKind::Success => "toast-success",
                            ToastKind::Info => "toast-info",
                            ToastKind::Warning => "toast-warning",
                            ToastKind::Error => "toast-error",
                        }
                    )}>
                        <i
                            class={classes!(
                                "mr-2",
                                "select-none",
                                "nf",
                                match t.kind() {
                                    ToastKind::Success => "nf-fa-ok_sign",
                                    ToastKind::Info => "nf-fa-circle_info",
                                    ToastKind::Warning => "nf-fa-warning",
                                    ToastKind::Error => "nf-fa-triangle_exclamation",
                                }
                            )}
                            aria-hidden="true"
                        />
                        <span class="flex-1">{t.message().clone()}</span>
                        <button
                            class="ml-3 opacity-80 hover:opacity-100"
                            onclick={handle_close_click}
                            aria-label="dismiss"
                        >
                            <i class="nf nf-fa-window_close" />
                        </button>
                    </div>
                }
            })}
        </div>
    }
}
