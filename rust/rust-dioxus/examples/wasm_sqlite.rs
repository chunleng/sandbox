use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::MessageEvent;
use web_sys::Worker;

const WORKER_JS: Asset = asset!(
    "/assets/sqlite.js",
    JsAssetOptions::new().with_minify(false) // dioxus minify does not work well with JS module files
);
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let worker = use_webworker();
    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        button {
            onclick: move |_| {
                worker.post_message(&"Init DB".into()).unwrap();
            },
            "Send Event"
        }
    }
}

#[cfg(target_family = "wasm")]
fn use_webworker() -> Worker {
    use web_sys::{WorkerOptions, WorkerType};

    use_hook(|| {
        let worker_options = WorkerOptions::new();
        worker_options.set_type(WorkerType::Module);
        let worker = Worker::new_with_options(&WORKER_JS.to_string(), &worker_options).unwrap();

        let onmessage: Closure<dyn FnMut(MessageEvent)> =
            Closure::wrap(Box::new(move |event: MessageEvent| {
                info!("Message received: {:?}", event.data());
            }));
        worker.set_onmessage(Some(&onmessage.as_ref().unchecked_ref()));

        // This closure ownership now belongs to JS
        onmessage.forget();
        worker
    })
}
