use wasm_bindgen::prelude::*;
use web_sys::{Element, HtmlElement};
use yew::prelude::*;

use crate::models::ToastKind;
use crate::{debug, system_error};

/// # Summary
///
/// ファイルサイズを分かり易い文字列に変換する
///
/// # Arguments
///
/// - `file_size`: ファイルサイズ
///
/// # Returns
///
/// - `String`: ファイルサイズ文字列
pub fn convert_file_size(file_size: u64) -> String {
    // ファイルサイズの単位
    const UNITS: [&str; 5] = ["B", "KiB", "MiB", "GiB", "TiB"];

    let mut size = file_size as f64;
    let mut i: usize = 0;
    let (size, i) = loop {
        if size < 1024.0 {
            break (size, i);
        }

        size /= 1024.0;
        i += 1;
    };
    let unit = UNITS[i];
    // 小数点第二位で丸める
    let size_rounded = (size * 100.0).round() / 100.0;

    // 小数部がほぼ 0 かどうかチェック
    if size_rounded.fract() < f64::EPSILON {
        format!("{size:.0} {unit}")
    } else {
        format!("{size:.2} {unit}")
    }
}

/// # Summary
///
/// イベントエレメントを指定したエレメントに変換する
///
/// # Arguments
///
/// - `e`: イベントエレメント
/// - `push_toast`: エラーメッセージ表示用のトースト
///
/// # Returns
///
/// - `Option<T>`: 変換後のエレメント
pub fn downcast<T>(e: &Event, push_toast: &Callback<(ToastKind, String)>) -> Option<T>
where
    T: wasm_bindgen::JsCast,
{
    let type_name_full = std::any::type_name::<T>();
    let type_name_short = type_name_full.rsplit("::").next().unwrap_or(type_name_full);

    e.target().and_then(|v| v.dyn_into::<T>().ok()).or_else(|| {
        system_error!(
            format!("Target element is not {type_name_short}"),
            push_toast
        );
        None
    })
}

/// # Summary
///
/// 指定したエレメントの高さ(padding, border, margin を含む)を取得する
///
/// # Arguments
///
/// - `node_ref`: 対象エレメントの NodeRef
///
/// # Returns
///
/// - `f64`: エレメントの高さ
pub fn get_element_height(node_ref: &NodeRef) -> f64 {
    match node_ref.cast::<Element>() {
        Some(element) => {
            // padding と border を含むエレメントの高さを取得
            let rect_height = element.get_bounding_client_rect().height();
            debug!(rect_height);

            // margin の高さ
            let mut margin_height: f64 = Default::default();

            if let Some(v) =
                web_sys::window().and_then(|v| v.get_computed_style(&element).ok().flatten())
            {
                let margin_top = v.get_property_value("margin-top").unwrap_or_default();
                debug!(margin_top);
                let margin_bottom = v.get_property_value("margin-bottom").unwrap_or_default();
                debug!(margin_bottom);
                let parse_margin = |v: String| {
                    v.trim()
                        .trim_end_matches("px")
                        .parse::<f64>()
                        .unwrap_or_default()
                };
                margin_height = parse_margin(margin_top) + parse_margin(margin_bottom);
            }

            rect_height + margin_height
        }
        None => Default::default(),
    }
}

/// # Summary
///
/// ファイルリスト表示領域の高さを計算し、設定する
///
/// # Arguments
///
/// - `header_ref`: <header> の NodeRef
/// - `footer_ref`: <footer> の NodeRef
pub fn set_content_height(header_ref: &NodeRef, footer_ref: &NodeRef) {
    let viewport_height = web_sys::window()
        .and_then(|v| v.visual_viewport().map(|v| v.height()))
        .or_else(|| {
            web_sys::window()
                .and_then(|v| v.inner_height().ok())
                .and_then(|v| v.as_f64())
        })
        .unwrap_or_default();
    debug!(viewport_height);

    // ファイルリスト表示領域の高さ
    let content_height =
        viewport_height - get_element_height(&header_ref) - get_element_height(&footer_ref);
    debug!(content_height);

    // <html> の style 属性にファイルリスト表示領域の高さをセット
    if let Some(v) = web_sys::window()
        .and_then(|v| v.document())
        .and_then(|v| v.document_element())
    {
        if let Some(html) = v.dyn_into::<HtmlElement>().ok() {
            let _ = html
                .style()
                .set_property("--content-height", &format!("{content_height}px"));
        }
    }
}
