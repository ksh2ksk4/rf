use rf_common::FileInfo;
use std::collections::HashSet;
use yew::prelude::*;

use crate::utils::*;

pub fn build_main(
    display_files: UseStateHandle<Vec<FileInfo>>,
    selected_files: UseStateHandle<HashSet<String>>,
    renaming_file: UseStateHandle<Option<String>>,
    handle_file_checkbox_click: Callback<Event>,
    handle_file_textbox_blur: Callback<FocusEvent>,
    handle_file_textbox_keypress: Callback<KeyboardEvent>,
    handle_file_anchor_click: Callback<MouseEvent>,
    handle_file_anchor_double_click: Callback<MouseEvent>,
) -> Html {
    html! {
        <main>
            <table class="file-list">
                <thead>
                    <tr>
                        <th>
                            <input
                                type="checkbox"
                                checked=false
                                aria-label="select all"
                            />
                        </th>
                        <th>{"name"}</th>
                        <th>{"size"}</th>
                        <th>{"created at"}</th>
                        <th>{"modified at"}</th>
                        <th>{"accessed at"}</th>
                    </tr>
                </thead>
                <tbody>
                    {for display_files.iter().map(|f| {
                        html! {
                            <tr class={if f.is_dir() {"dir"} else {"file"}}>
                                <td class="select-file">
                                    <input
                                        type="checkbox"
                                        checked={(*selected_files).contains(&f.path())}
                                        onchange={handle_file_checkbox_click.clone()}
                                        data-path={f.path().clone()}
                                        aria-label="select file"
                                    />
                                </td>
                                <td class="name">
                                    {if f.is_dir() {
                                        html! {<i class="line-start folder nf nf-fa-folder" />}
                                    } else {
                                        html! {<i class="line-start file nf nf-fa-file" />}
                                    }}
                                    {if (*renaming_file).as_ref().map(|v| v == &f.path()).unwrap_or(false) {
                                        html! {
                                            <input
                                                id="renaming_file"
                                                type="text"
                                                value={f.name().clone()}
                                                onblur={handle_file_textbox_blur.clone()}
                                                onkeypress={handle_file_textbox_keypress.clone()}
                                                data-path={f.path().clone()}
                                            />
                                        }
                                    } else {
                                        html! {
                                            <a
                                                href="#"
                                                onclick={handle_file_anchor_click.clone()}
                                                ondblclick={handle_file_anchor_double_click.clone()}
                                                data-is-dir={f.is_dir().to_string()}
                                                data-path={f.path().clone()}
                                            >
                                                {&f.name()}
                                            </a>
                                        }
                                    }}
                                </td>
                                <td class="size">{convert_file_size(f.size())}</td>
                                <td class="datetime">{&f.created()}</td>
                                <td class="datetime">{&f.modified()}</td>
                                <td class="datetime">{&f.accessed()}</td>
                            </tr>
                        }
                    })}
                </tbody>
            </table>
        </main>
    }
}
