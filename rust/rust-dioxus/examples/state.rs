use std::rc::Rc;

use dioxus::{logger::tracing, prelude::*};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        NodeRef {}
    }
}

#[component]
fn NodeRef() -> Element {
    let mut input_node_ref = use_signal::<Option<Rc<MountedData>>>(|| None);
    let mut input_value = use_signal::<Option<String>>(|| None);
    let fmt_value = || input_value.read().clone().unwrap_or("".to_string());

    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "icon", href: FAVICON }
        input {
            class: "border-black border",
            r#type: "text",
            oninput: move |evt| {
                tracing::info!("{:?}", evt);
                input_value.set(Some(evt.value()));
            },
            onmounted: move |evt| { input_node_ref.set(Some(evt.data)) },
        }
        div { class: "bg-white font-bold bg-black ml-3 ", "Value in input: {fmt_value()}" }
    }
}
