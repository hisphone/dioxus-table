use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn DataGrid(cx: Scope) -> Element {
    cx.render(rsx!(
        table {
            caption {}

        }
    ))
}
