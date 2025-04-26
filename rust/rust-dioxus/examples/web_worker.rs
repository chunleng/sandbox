use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::{MessageEvent, SharedWorker, Worker};

const WORKER_JS: Asset = asset!("/assets/worker.js");
const SHAREDWORKER_JS: Asset = asset!("/assets/sharedworker.js");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let worker = use_webworker();
    let shared_worker = use_shared_webworker();

    rsx! {
        button {
            onclick: move |_| {
                worker.post_message(&"This is sent from dioxus!".into()).unwrap();
            },
            "Send Event"
        }
        button {
            onclick: move |_| {
                shared_worker
                    .port()
                    .post_message(
                        &"This is sent from dioxus to all tabs with this page opened".into(),
                    )
                    .unwrap();
            },
            "Send Event to All Tabs/Windows"
        }
    }
}

#[cfg(target_family = "wasm")]
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

#[cfg(target_family = "wasm")]
fn use_shared_webworker() -> SharedWorker {
    use_hook(|| {
        let worker = SharedWorker::new(&SHAREDWORKER_JS.to_string()).unwrap();
        worker.port().start();

        let onmessage: Closure<dyn FnMut(MessageEvent)> =
            Closure::wrap(Box::new(move |event: MessageEvent| {
                info!("Message received: {:?}", event.data())
            }));
        worker
            .port()
            .set_onmessage(Some(&onmessage.as_ref().unchecked_ref()));

        // This closure ownership now belongs to JS
        onmessage.forget();
        worker
    })
}
