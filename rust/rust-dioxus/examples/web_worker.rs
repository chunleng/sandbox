use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::{MessageEvent, Worker};

const WORKER_JS: Asset = asset!("/assets/worker.js");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let worker = use_webworker();

    rsx! {
        button {
            onclick: move |_| {
                worker.post_message(&"This is sent from dioxus!".into()).unwrap();
            },
            "Send Event"
        }
    }
}

fn use_webworker() -> Worker {
    use_hook(|| {
        let worker = Worker::new(&WORKER_JS.to_string()).unwrap();

        let onmessage: Closure<dyn FnMut(MessageEvent)> =
            Closure::wrap(Box::new(move |event: MessageEvent| {
                info!("Message received: {:?}", event.data())
            }));
        worker.set_onmessage(Some(&onmessage.as_ref().unchecked_ref()));

        // This closure ownership now belongs to JS
        onmessage.forget();
        worker
    })
}
