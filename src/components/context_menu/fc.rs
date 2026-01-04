use yew::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    /// # Summary
    ///
    /// インスタンスを生成
    ///
    /// # Returns
    ///
    /// - `Self`: インスタンス
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, PartialEq)]
pub struct ContextMenuData {
    pub coordinate: Coordinate,
    pub path: String,
    pub item_1: &'static str,
    pub item_2: &'static str,
    pub item_3: &'static str,
    pub item_4: &'static str,
    pub item_5: &'static str,
    pub item_6: &'static str,
    pub item_7: &'static str,
    pub item_8: &'static str,
    pub item_9: &'static str,
}

impl ContextMenuData {
    /// # Summary
    ///
    /// インスタンスを生成
    ///
    /// # Returns
    ///
    /// - `Self`: インスタンス
    pub fn new(coordinate: Coordinate, path: impl Into<String>) -> Self {
        Self {
            coordinate,
            path: path.into(),
            item_1: "Open",
            item_2: "Open with",
            item_3: "Rename",
            item_4: "Copy",
            item_5: "Paste",
            item_6: "Move",
            item_7: "Copy as path",
            item_8: "Trash",
            item_9: "Properties",
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct ContextMenuProps {
    pub context_menu_data: UseStateHandle<Option<ContextMenuData>>,
}

/// # Summary
///
/// コンテキストメニューを生成する
///
/// # Returns
///
/// `Html`: HTML
#[function_component(ContextMenu)]
pub fn context_menu_component(props: &ContextMenuProps) -> Html {
    //
    // アプリ共有のステート
    //
    let context_menu_data = &props.context_menu_data;

    if let Some(v) = context_menu_data.as_ref() {
        let x = v.coordinate.x;
        let y = v.coordinate.y;
        html! {
            <div
                class="context-menu"
                onclick={
                    // コンテキストメニュー内部のクリックではイベントを伝播させず、
                    // <main> の onclick で処理されないようにする
                    Callback::from(|e: MouseEvent| e.stop_propagation())
                }
                style={format!("position: absolute; left: {x}px; top: {y}px; z-index: 100;")}
            >
                <ul>
                    <li>
                        <i
                            class="fa-solid fa-circle-play"
                            aria-hidden="true"
                        />
                        {v.item_1}
                    </li>
                    <li>
                        <i
                            class="fa-regular fa-circle-play"
                            aria-hidden="true"
                        />
                        {v.item_2}
                    </li>
                    <li>
                        <i
                            class="fa-solid fa-keyboard"
                            aria-hidden="true"
                        />
                        {v.item_3}
                    </li>
                    <li>
                        <i
                            class="fa-solid fa-copy"
                            aria-hidden="true"
                        />
                        {v.item_4}
                    </li>
                    <li>
                        <i
                            class="fa-solid fa-paste"
                            aria-hidden="true"
                        />
                        {v.item_5}
                    </li>
                    <li>
                        <i
                            class="fa-solid fa-file-export"
                            aria-hidden="true"
                        />
                        {v.item_6}
                    </li>
                    <li>
                        <i
                            class="fa-solid fa-p"
                            aria-hidden="true"
                        />
                        {v.item_7}
                    </li>
                    <li>
                        <i
                            class="fa-solid fa-trash"
                            aria-hidden="true"
                        />
                        {v.item_8}
                    </li>
                    <li>
                        <i
                            class="fa-solid fa-list"
                            aria-hidden="true"
                        />
                        {v.item_9}
                    </li>
                </ul>
            </div>
        }
    } else {
        html! {}
    }
}
